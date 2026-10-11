//! The resting order book — what is on offer, in the order matching will
//! consume it. No matching logic: the book hands out resting orders best
//! first, so `exchange_match` takes from the front and never decides who is
//! next.
//!
//! # The priority rule
//!
//! Price first, then arrival: a better price always ranks ahead, and within
//! one price the earlier arrival — FIFO, no exceptions. Better is higher for a
//! bid, lower for an ask ([`exchange_side::side_ahead`]).
//!
//! Arrival is an `exchange_seq::Sequence` the caller claims, never a clock
//! reading: two orders placed in the same instant still get distinct
//! positions, and a replay reproduces them. Should two share an arrival, the
//! one inserted first stays ahead. Nothing here iterates a hash or compares
//! addresses.
//!
//! # One book per instrument
//!
//! [`Book`] keeps each instrument's bids and asks apart — hard problem 1.
//! [`insert`](Book::insert) reads the instrument off the order; every other
//! per-instrument method takes it as its first argument. An instrument nothing
//! has rested on behaves as an empty book. [`iter`](Book::iter),
//! [`len`](Book::len) and [`is_empty`](Book::is_empty) span every instrument:
//! `exchange_core::Exchange::cancel` finds an order by id alone.
//!
//! # Representation
//!
//! A `Vec` of `( InstrumentId, InstrumentBook )` pairs sorted by instrument;
//! each side a [`VecDeque`] of [`Level`]s, best first, so an emptied best level
//! leaves in O(1). Within a level, [`exchange_level`] keeps arrival order.
//! Instruments and prices are found by `partition_point`. [`Resting`] is an
//! alias of [`exchange_level::LevelNode`].

use std::collections::VecDeque;

use exact_arith::{ Price, Quantity };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_level::{ Level, level_empty_is, level_len, level_new, level_pop_front, level_push, level_remove };
use exchange_side::{ Side, side_ahead };

/// An order on the book, with what is left of it.
pub type Resting = exchange_level::LevelNode;

/// One instrument's own two sides.
#[ derive( Debug, Clone, Default, PartialEq, Eq ) ]
struct InstrumentBook
{
  /// Buy levels, highest price first; FIFO arrival within each.
  bids : VecDeque< Level >,
  /// Sell levels, lowest price first; FIFO arrival within each.
  asks : VecDeque< Level >,
}

/// Every instrument's resting orders, kept apart — see the module doc's
/// "One book per instrument" section.
#[ derive( Debug, Clone, Default ) ]
pub struct Book
{
  per_instrument : Vec< ( InstrumentId, InstrumentBook ) >,
  /// The highest id ever seated. No resting id exceeds it, so an id above it
  /// is no duplicate — the check every caller minting ids in order hits.
  id_high : Option< OrderId >,
}

/// Equal when the same orders rest the same way. `id_high` is a cache of
/// past inserts, and an emptied instrument's slot a leftover of them; neither
/// is part of what rests.
//
// Fix(an_emptied_instrument_made_two_equal_books_unequal):
// Root cause: a slot outlives its instrument's last order, and `eq` compared
// the slots, so a book cancelled back to empty differed from `Book::new()`.
// Pitfall: excluding `id_high` covered one leftover of history, not all.
impl PartialEq for Book
{
  fn eq( &self, other : &Self ) -> bool
  {
    self.live().eq( other.live() )
  }
}

impl Eq for Book {}

impl Book
{
  /// An empty book.
  #[ must_use ]
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Place `resting` at its priority position.
  ///
  /// Inserted rather than appended-and-sorted, so the book is in priority
  /// order at every observable point rather than only after a sort someone
  /// has to remember to call.
  ///
  /// Returns `false`, refusing the insert, if `resting.remaining` is zero or
  /// `resting.order.id` already rests on either side — the same "caller
  /// error that would silently corrupt the book" class [`Self::consume_best`]
  /// guards.
  //
  // Fix(book_insert_had_no_duplicate_order_id_rejection):
  // Root cause: `insert` seated a second order under an id already resting.
  // `cancel` removes only the first match, so the duplicate outlived its own
  // cancellation and kept trading under the same identity.
  // Pitfall: both in-repo callers mint unique ids, but `Book` is public; its
  // contract must hold for any caller.
  //
  // Fix(book_insert_had_no_zero_remaining_rejection):
  // Root cause: `insert` seated a zero-remaining order. Only `consume_best`
  // evicts an order, and only when its own subtraction reaches zero, so the
  // phantom stayed at the front and every later `consume_best` against it
  // underflowed and returned `false`.
  // Pitfall: as above — a guard in every caller is not a guard in `Book`.
  #[ must_use ]
  pub fn insert( &mut self, resting : Resting ) -> bool
  {
    let id = resting.order.id;
    let maybe_seated = self.id_high.is_some_and( | high | id <= high );
    if resting.remaining == Quantity::ZERO || ( maybe_seated && self.contains_id( id ) )
    {
      return false;
    }
    self.id_high = self.id_high.max( Some( id ) );

    let side = resting.order.side;
    let price = resting.order.price;
    let book = Self::find_or_create_mut( &mut self.per_instrument, resting.order.instrument );
    let levels = Self::side_mut( book, side );
    let at = levels.partition_point( | level | side_ahead( side, level.price, price ) );

    if levels.get( at ).is_some_and( | level | level.price == price )
    {
      // Not `level_push`, which appends: callers may insert out of arrival
      // order (`t02_time_priority_within_one_price_level`), so the node goes
      // to its arrival's position.
      //
      // Fix(an_insert_sharing_an_arrival_jumped_the_queue):
      // Root cause: the position was found with `<`, which lands before
      // every node of equal arrival, so ties came out newest first.
      // Pitfall: the one caller that claims arrivals itself never ties;
      // `exchange_inbound::inbound_apply` rests whatever its producer stamped.
      let nodes = &mut levels[ at ].nodes;
      let node_at = nodes.partition_point( | node | node.arrival <= resting.arrival );
      nodes.insert( node_at, resting );
    }
    else
    {
      let mut level = level_new( price );
      level_push( &mut level, resting );
      levels.insert( at, level );
    }
    true
  }

  /// Whether an order with this id rests on any instrument's either side.
  fn contains_id( &self, id : OrderId ) -> bool
  {
    self.iter().any( | resting | resting.order.id == id )
  }

  /// Remove the order with this id from `instrument`'s book, whichever side
  /// it rests on.
  ///
  /// Returns what was removed, or [`None`] if no such order rests — which is
  /// a race result rather than a caller error, since the order may have filled
  /// or been cancelled already, or `instrument` itself has never had an
  /// order rest on it. Drops the level too, if removing its last node
  /// empties it — a level never persists empty.
  ///
  /// Each side's best order is checked first and removed in O(1). Any other
  /// order costs a walk of the instrument's book, bids before asks.
  pub fn cancel( &mut self, instrument : InstrumentId, id : OrderId ) -> Option< Resting >
  {
    let book = Self::find_mut( &mut self.per_instrument, instrument )?;
    Self::try_cancel_best( book, id ).or_else( || Self::try_cancel_walk( book, id ) )
  }

  /// [`Self::cancel`] when `id` is either side's best order: removed in O(1).
  /// [`None`] if it is not, though it may still rest further back.
  ///
  /// `insert` refuses a duplicate id, so a front match is the order
  /// [`Self::try_cancel_walk`] would find.
  fn try_cancel_best( book : &mut InstrumentBook, id : OrderId ) -> Option< Resting >
  {
    for levels in [ &mut book.bids, &mut book.asks ]
    {
      let Some( level ) = levels.front_mut()
      else
      {
        continue;
      };
      if level.nodes.front().is_some_and( | best | best.order.id == id )
      {
        let removed = level_pop_front( level );
        if level_empty_is( level )
        {
          levels.pop_front();
        }
        return removed;
      }
    }
    None
  }

  /// [`Self::cancel`] for any order: a walk of `book`, bids before asks.
  ///
  /// Keeps its own copy of remove-and-drop-level rather than sharing one with
  /// [`Self::try_cancel_best`]: a shared helper made cancels that miss the front
  /// 38% slower (146 ms vs 106 ms for 10 000, release build).
  fn try_cancel_walk( book : &mut InstrumentBook, id : OrderId ) -> Option< Resting >
  {
    for levels in [ &mut book.bids, &mut book.asks ]
    {
      for at in 0..levels.len()
      {
        let Some( removed ) = level_remove( &mut levels[ at ], id )
        else
        {
          continue;
        };

        if level_empty_is( &levels[ at ] )
        {
          levels.remove( at );
        }
        return Some( removed );
      }
    }
    None
  }

  /// One side of `instrument`'s book, in priority order — the first item is
  /// next to be consumed. Empty if `instrument` has never had an order rest
  /// on it, the same as an instrument whose every order already left.
  pub fn side( &self, instrument : InstrumentId, side : Side ) -> impl Iterator< Item = &Resting >
  {
    self.levels( instrument, side ).iter().flat_map( | level | level.nodes.iter() )
  }

  /// The best order on `instrument`'s `side`, or [`None`] if that side — or
  /// the whole instrument — is empty.
  #[ must_use ]
  pub fn best( &self, instrument : InstrumentId, side : Side ) -> Option< &Resting >
  {
    self.levels( instrument, side ).front()?.nodes.front()
  }

  /// Reduce the best order on `instrument`'s `side` by `taken`, removing it
  /// if that empties it.
  ///
  /// Returns `false` if the side (or the instrument itself) is empty, or
  /// `taken` exceeds what rests there — all of which are caller errors that
  /// would silently corrupt the book, so none is allowed to pass quietly.
  pub fn consume_best( &mut self, instrument : InstrumentId, side : Side, taken : Quantity ) -> bool
  {
    let Some( book ) = Self::find_mut( &mut self.per_instrument, instrument )
    else
    {
      return false;
    };
    let levels = Self::side_mut( book, side );
    let Some( level ) = levels.front_mut()
    else
    {
      return false;
    };

    let best = level.nodes.front_mut()
      .expect( "a level is removed the instant it empties; it never rests here with zero nodes" );

    let Ok( left ) = best.remaining.checked_sub( taken )
    else
    {
      return false;
    };

    if left == Quantity::ZERO
    {
      level_pop_front( level );
      if level_empty_is( level )
      {
        levels.pop_front();
      }
    }
    else
    {
      best.remaining = left;
    }
    true
  }

  /// How many orders rest, every instrument and side together.
  #[ must_use ]
  pub fn len( &self ) -> usize
  {
    self.per_instrument.iter()
    .flat_map( | ( _, book ) | book.bids.iter().chain( book.asks.iter() ) )
    .map( level_len )
    .sum()
  }

  /// Whether nothing rests anywhere, on any instrument.
  #[ must_use ]
  pub fn is_empty( &self ) -> bool
  {
    self.per_instrument.iter().all( | ( _, book ) | book.bids.is_empty() && book.asks.is_empty() )
  }

  /// Every resting order, across every instrument — bids then asks within
  /// each, each side in priority order.
  pub fn iter( &self ) -> impl Iterator< Item = &Resting >
  {
    self.per_instrument.iter()
    .flat_map( | ( _, book ) | book.bids.iter().chain( book.asks.iter() ) )
    .flat_map( | level | level.nodes.iter() )
  }

  /// How many orders currently rest at exactly `price` on `instrument`'s
  /// `side` — what `exchange_cap::cap_check_rest` needs as `current_rests`.
  ///
  /// Zero if no level exists at that price — a level never persists empty
  /// (see [`Self::cancel`]), so "no level" and "an empty level" are the same
  /// observable state.
  #[ must_use ]
  pub fn rests_at( &self, instrument : InstrumentId, side : Side, price : Price ) -> usize
  {
    self.levels( instrument, side ).iter().find( | level | level.price == price ).map_or( 0, level_len )
  }

  /// How many distinct price levels currently exist on `instrument`'s
  /// `side` — what `exchange_cap::cap_check_level` needs as `current_levels`.
  #[ must_use ]
  pub fn level_count( &self, instrument : InstrumentId, side : Side ) -> usize
  {
    self.levels( instrument, side ).len()
  }

  /// How many orders `account` currently rests on `instrument`'s book, both
  /// sides — what `exchange_cap::cap_check_account` needs as
  /// `current_account_rests`. A walk of the instrument's book.
  #[ must_use ]
  pub fn account_rests( &self, instrument : InstrumentId, account : AccountId ) -> usize
  {
    self.side( instrument, Side::Buy ).chain( self.side( instrument, Side::Sell ) )
      .filter( | resting | resting.order.account == account )
      .count()
  }

  /// Every instrument slot with an order resting.
  fn live( &self ) -> impl Iterator< Item = &( InstrumentId, InstrumentBook ) >
  {
    self.per_instrument.iter().filter( | ( _, book ) | !book.bids.is_empty() || !book.asks.is_empty() )
  }

  /// `instrument`'s own levels on `side`, or no levels if nothing has ever
  /// rested on that instrument.
  fn levels( &self, instrument : InstrumentId, side : Side ) -> &VecDeque< Level >
  {
    static NO_LEVELS : VecDeque< Level > = VecDeque::new();
    let Some( book ) = Self::find( &self.per_instrument, instrument )
    else
    {
      return &NO_LEVELS;
    };
    match side
    {
      Side::Buy => &book.bids,
      Side::Sell => &book.asks,
    }
  }

  /// `book`'s own levels on `side`.
  fn side_mut( book : &mut InstrumentBook, side : Side ) -> &mut VecDeque< Level >
  {
    match side
    {
      Side::Buy => &mut book.bids,
      Side::Sell => &mut book.asks,
    }
  }

  /// `instrument`'s slot in `per_instrument`, or [`None`] if nothing has
  /// ever rested on it. `per_instrument` stays sorted by
  /// [`InstrumentId`] — never a hash — so this is the same
  /// [`partition_point`](slice::partition_point) lookup every side already
  /// uses for price.
  fn find( per_instrument : &[ ( InstrumentId, InstrumentBook ) ], instrument : InstrumentId ) -> Option< &InstrumentBook >
  {
    let at = per_instrument.partition_point( | ( id, _ ) | *id < instrument );
    per_instrument.get( at ).filter( | ( id, _ ) | *id == instrument ).map( | ( _, book ) | book )
  }

  /// [`Self::find`], mutably.
  fn find_mut( per_instrument : &mut [ ( InstrumentId, InstrumentBook ) ], instrument : InstrumentId ) -> Option< &mut InstrumentBook >
  {
    let at = per_instrument.partition_point( | ( id, _ ) | *id < instrument );
    per_instrument.get_mut( at ).filter( | ( id, _ ) | *id == instrument ).map( | ( _, book ) | book )
  }

  /// `instrument`'s slot, creating an empty one at its sorted position if
  /// this is its first ever mention — the same way a fresh price creates its
  /// own level the first time an order rests at it.
  fn find_or_create_mut( per_instrument : &mut Vec< ( InstrumentId, InstrumentBook ) >, instrument : InstrumentId ) -> &mut InstrumentBook
  {
    let at = per_instrument.partition_point( | ( id, _ ) | *id < instrument );
    if per_instrument.get( at ).is_none_or( | ( id, _ ) | *id != instrument )
    {
      per_instrument.insert( at, ( instrument, InstrumentBook::default() ) );
    }
    &mut per_instrument[ at ].1
  }
}
