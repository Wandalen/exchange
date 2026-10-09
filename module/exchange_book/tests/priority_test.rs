//! Test Matrix T02–T04 — the priority rule, from both sides, and cancel.
//!
//! Every assertion here reads the book's *published order* rather than its
//! internals, because that order is the entire contract: matching takes from
//! the front and never re-decides. A book that stores the right orders in the
//! wrong sequence is a book that fills the wrong people, and nothing
//! downstream can notice.

use exact_arith::{ Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

/// The one instrument every test in this file threads through `Book`'s now
/// instrument-scoped methods — T05's own leakage test is the one place a
/// second id is deliberately used instead.
const INSTRUMENT : InstrumentId = InstrumentId( 1 );

fn rest( id : u64, side : Side, price : &str, quantity : i64, arrival : u64 ) -> Resting
{
  rest_on( INSTRUMENT, id, side, price, quantity, arrival )
}

fn rest_on( instrument : InstrumentId, id : u64, side : Side, price : &str, quantity : i64, arrival : u64 ) -> Resting
{
  let quantity = Quantity::from_int( quantity ).unwrap();
  Resting
  {
    order : Order
    {
      id : OrderId( id ),
      instrument,
      account : AccountId( id ),
      side,
      price : Price::parse( price ).unwrap(),
      quantity,
      tif : Tif::Gtc,
      client : None,
    },
    remaining : quantity,
    arrival : Sequence( arrival ),
  }
}

fn ids( book : &Book, side : Side ) -> Vec< u64 >
{
  book.side( INSTRUMENT, side ).map( | resting | resting.order.id.0 ).collect()
}

/// T02 — same price, earlier arrival rests ahead.
///
/// Inserted newest-first so that an implementation which merely appends would
/// produce the reverse and be caught. Appending is the natural mistake and it
/// is invisible until two orders share a price.
#[ test ]
fn t02_time_priority_within_one_price_level()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 2, Side::Buy, "2.50", 5, 20 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 1, Side::Buy, "2.50", 5, 10 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 3, Side::Buy, "2.50", 5, 30 ) ), "fresh id in this test, must not already rest" );

  assert_eq!( ids( &book, Side::Buy ), vec![ 1, 2, 3 ], "earliest arrival first, whatever the insertion order" );
}

/// T03 — a better price rests ahead, whenever it arrived.
///
/// For bids "better" means higher, and the newest order here is also the most
/// aggressive, so an implementation ranking purely by arrival would put it
/// last and be caught.
#[ test ]
fn t03_price_priority_across_levels_for_bids()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "2.00", 5, 10 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 2, Side::Buy, "1.00", 5, 20 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 3, Side::Buy, "3.00", 5, 30 ) ), "fresh id in this test, must not already rest" );

  assert_eq!( ids( &book, Side::Buy ), vec![ 3, 1, 2 ], "highest bid first" );
}

/// T03, the other direction — for asks "better" means lower.
///
/// Tested separately rather than assumed symmetric: the sides are the one
/// asymmetry in the crate, and a copy-paste that left both comparing the same
/// way would pass the bid test alone.
#[ test ]
fn t03_price_priority_across_levels_for_asks()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Sell, "2.00", 5, 10 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 2, Side::Sell, "3.00", 5, 20 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 3, Side::Sell, "1.00", 5, 30 ) ), "fresh id in this test, must not already rest" );

  assert_eq!( ids( &book, Side::Sell ), vec![ 3, 1, 2 ], "lowest ask first" );
}

/// Price outranks time, not the other way round.
///
/// The two rules only conflict when a later order is priced better, and this
/// is the case that says which one wins. An implementation sorting by
/// `( arrival, price )` passes T02 and T03 individually and fails here.
#[ test ]
fn price_outranks_arrival_when_the_two_disagree()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "2.00", 5, 10 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 2, Side::Buy, "2.50", 5, 99 ) ), "fresh id in this test, must not already rest" );

  assert_eq!( ids( &book, Side::Buy ), vec![ 2, 1 ], "the later, better-priced order rests ahead" );
}

/// T04 — a cancelled order leaves, and the rest keep their order.
#[ test ]
fn t04_cancel_removes_exactly_one_order()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "2.50", 5, 10 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 2, Side::Buy, "2.50", 5, 20 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 3, Side::Buy, "2.50", 5, 30 ) ), "fresh id in this test, must not already rest" );

  let removed = book.cancel( INSTRUMENT, OrderId( 2 ) ).expect( "order 2 rests" );

  assert_eq!( removed.order.id, OrderId( 2 ) );
  assert_eq!( ids( &book, Side::Buy ), vec![ 1, 3 ], "the survivors keep their relative order" );
}

/// T04 — cancelling what is not there is a defined outcome, not a panic.
///
/// It is a race result: the order may have filled a moment earlier. A crate
/// that panicked here would turn a normal event into an outage.
#[ test ]
fn t04_cancelling_an_absent_order_reports_rather_than_panics()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "2.50", 5, 10 ) ), "fresh id in this test, must not already rest" );

  assert!( book.cancel( INSTRUMENT, OrderId( 99 ) ).is_none() );
  assert_eq!( book.len(), 1, "and nothing else moved" );
}

/// Cancel finds an order on either side without being told which.
#[ test ]
fn cancel_searches_both_sides()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "2.00", 5, 10 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 2, Side::Sell, "3.00", 5, 20 ) ), "fresh id in this test, must not already rest" );

  assert!( book.cancel( INSTRUMENT, OrderId( 2 ) ).is_some() );
  assert_eq!( ids( &book, Side::Sell ), Vec::< u64 >::new() );
  assert_eq!( ids( &book, Side::Buy ), vec![ 1 ] );
}

/// `insert` refuses a second `Resting` sharing an id that already rests,
/// rather than silently letting two orders trade under one identity.
///
/// `cancel` can only ever remove one match for an id — `position()` stops at
/// the first — so an unrejected duplicate would silently outlive its own
/// cancellation. Checked cross-side too: the duplicate here arrives on the
/// opposite side from the original, which a same-side-only check would miss.
#[ test ]
fn insert_refuses_a_duplicate_order_id()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "2.50", 5, 10 ) ), "fresh id, must succeed" );

  assert!( !book.insert( rest( 1, Side::Sell, "3.00", 2, 20 ) ), "id 1 already rests, on the other side even" );

  assert_eq!( book.len(), 1, "the rejected insert left no trace" );
  assert_eq!( ids( &book, Side::Buy ), vec![ 1 ], "the original resting order is untouched" );
  assert_eq!( ids( &book, Side::Sell ), Vec::< u64 >::new(), "the rejected order never landed" );
}

/// Root Cause: `Book::insert` placed every `Resting` handed to it with no
/// check that `remaining` was above zero, even though `Resting::remaining`'s
/// own doc comment states the invariant "always greater than zero — a
/// resting order with nothing left is removed rather than kept at zero."
/// `consume_best` only ever removes a zero-remaining order from inside a
/// `checked_sub` that just reached zero — a zero-remaining `Resting` seated
/// directly by `insert` never passes through that arithmetic, so nothing
/// ever evicts it.
/// Why Not Caught: every `insert` call in this file builds its `Resting`
/// through `rest(...)`, which always derives `remaining` from a positive
/// `quantity` argument — none of them ever tried to seat a `Resting` whose
/// `remaining` was already `Quantity::ZERO`.
/// Fix Applied: `insert` now refuses (`false`) a `Resting` whose `remaining`
/// is `Quantity::ZERO`, the same way it already refuses a duplicate
/// `order.id`.
/// Prevention: a struct field's doc comment stating an invariant is a claim
/// about every value of that type, not only the ones today's callers happen
/// to construct — a public constructor or insertion point for that type
/// must enforce it, not assume it.
/// Pitfall: a zero-remaining order that does get seated is not merely inert
/// — it jams the book. `consume_best` subtracts `taken` from `remaining` via
/// `checked_sub`, which underflows and returns `false` the moment `taken` is
/// positive against an already-zero `remaining`, so the phantom order is
/// never evicted and sits at the front blocking every future match against
/// that side.
#[ test ]
fn insert_refuses_a_zero_remaining_order()
{
  let mut book = Book::new();
  let mut nothing_left = rest( 1, Side::Buy, "2.50", 5, 10 );
  nothing_left.remaining = Quantity::ZERO;

  assert!( !book.insert( nothing_left ), "a resting order with nothing left must never be seated" );
  assert!( book.is_empty(), "the refused insert must leave no trace" );
}

/// Consuming part of the best order keeps it at the front with less left.
#[ test ]
fn a_partly_consumed_order_keeps_its_place()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Sell, "2.50", 10, 10 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 2, Side::Sell, "2.50", 10, 20 ) ), "fresh id in this test, must not already rest" );

  assert!( book.consume_best( INSTRUMENT, Side::Sell, Quantity::from_int( 4 ).unwrap() ) );

  assert_eq!( ids( &book, Side::Sell ), vec![ 1, 2 ], "it did not lose priority by being partly filled" );
  assert_eq!( book.best( INSTRUMENT, Side::Sell ).unwrap().remaining, Quantity::from_int( 6 ).unwrap() );
}

/// Consuming all of the best order removes it.
#[ test ]
fn a_fully_consumed_order_leaves_the_book()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Sell, "2.50", 10, 10 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 2, Side::Sell, "2.50", 10, 20 ) ), "fresh id in this test, must not already rest" );

  assert!( book.consume_best( INSTRUMENT, Side::Sell, Quantity::from_int( 10 ).unwrap() ) );

  assert_eq!( ids( &book, Side::Sell ), vec![ 2 ], "an emptied order is removed, not kept at zero" );
}

/// Consuming more than rests is refused rather than silently clamped.
///
/// A clamp would leave the book and its caller disagreeing about how much
/// changed hands, which is the shape of a bug that balances perfectly and
/// still hands the wrong quantity to someone.
#[ test ]
fn consuming_more_than_rests_is_refused()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Sell, "2.50", 4, 10 ) ), "fresh id in this test, must not already rest" );

  assert!( !book.consume_best( INSTRUMENT, Side::Sell, Quantity::from_int( 5 ).unwrap() ) );
  assert_eq!( book.best( INSTRUMENT, Side::Sell ).unwrap().remaining, Quantity::from_int( 4 ).unwrap(), "and nothing moved" );
  assert!( !book.consume_best( INSTRUMENT, Side::Buy, Quantity::EPSILON ), "nor on an empty side" );
}

/// The two sides are independent — an order never appears on the wrong one.
#[ test ]
fn the_sides_do_not_leak_into_each_other()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "2.00", 5, 10 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 2, Side::Sell, "3.00", 5, 20 ) ), "fresh id in this test, must not already rest" );

  assert_eq!( ids( &book, Side::Buy ), vec![ 1 ] );
  assert_eq!( ids( &book, Side::Sell ), vec![ 2 ] );
  assert_eq!( book.len(), 2 );
  assert!( !book.is_empty() );
  assert_eq!( book.iter().count(), 2 );
}

/// `iter()` publishes bids before asks, each side still in its own priority
/// order — not merely the right count.
///
/// The check above this one reads only `.count()`, which cannot tell bids
/// from asks or notice a reordering: `asks().chain(bids())`, or any
/// interleaving of the two, would pass it exactly as well. This reads the
/// actual sequence `iter()`'s own doc comment promises — bids first, then
/// asks, each ranked by the same priority rule `side()` already publishes.
#[ test ]
fn iter_publishes_bids_then_asks_each_still_in_priority_order()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "1.00", 5, 10 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 2, Side::Buy, "2.00", 5, 20 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 3, Side::Sell, "5.00", 5, 30 ) ), "fresh id in this test, must not already rest" );
  assert!( book.insert( rest( 4, Side::Sell, "4.00", 5, 40 ) ), "fresh id in this test, must not already rest" );

  let sequence : Vec< u64 > = book.iter().map( | resting | resting.order.id.0 ).collect();
  assert_eq!( sequence, vec![ 2, 1, 4, 3 ], "bids best-first, then asks best-first" );
}

/// Arrival positions break every tie, so the published order is total.
///
/// The property behind T02 rather than an instance of it: whatever prices and
/// arrivals are inserted, no two neighbours are ever indistinguishable — which
/// is what stops a tie from being resolved by something the sequence does not
/// carry.
#[ test ]
fn the_published_order_is_total_with_no_ties_left()
{
  let mut book = Book::new();
  for ( id, price, arrival ) in [ ( 1, "2.50", 30 ), ( 2, "2.50", 10 ), ( 3, "1.50", 20 ), ( 4, "2.50", 20 ) ]
  {
    assert!( book.insert( rest( id, Side::Buy, price, 5, arrival ) ), "fresh id in this test, must not already rest" );
  }

  let published : Vec< _ > = book.side( INSTRUMENT, Side::Buy ).collect();
  for pair in published.windows( 2 )
  {
    let ( ahead, behind ) = ( &pair[ 0 ], &pair[ 1 ] );
    let ordered = ahead.order.price > behind.order.price
    || ( ahead.order.price == behind.order.price && ahead.arrival < behind.arrival );
    assert!( ordered, "order {} and {} are not strictly ordered", ahead.order.id.0, behind.order.id.0 );
  }
  assert_eq!( ids( &book, Side::Buy ), vec![ 2, 4, 1, 3 ] );
}

/// One [`Book`] holds every instrument's own orders, but never mixes them —
/// hard problem 1, and the one test in this file that deliberately uses a
/// second instrument id instead of the shared `INSTRUMENT` constant.
#[ test ]
fn orders_on_one_instrument_never_leak_into_another()
{
  let other = InstrumentId( 2 );
  let mut book = Book::new();
  assert!( book.insert( rest_on( INSTRUMENT, 1, Side::Buy, "2.00", 5, 10 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest_on( other, 2, Side::Buy, "9.00", 5, 20 ) ), "fresh id, must succeed — a better price on a different instrument" );

  // Each instrument sees only its own order, including at its own best
  // price — instrument 2's far-better price never surfaces on instrument 1's
  // side, which is exactly what a flat, instrument-agnostic book would get
  // wrong.
  assert_eq!( ids( &book, Side::Buy ), vec![ 1 ], "instrument 1's side names only its own order" );
  assert_eq!( book.side( other, Side::Buy ).map( | resting | resting.order.id.0 ).collect::< Vec< _ > >(), vec![ 2 ] );
  assert_eq!( book.best( INSTRUMENT, Side::Buy ).unwrap().order.id, OrderId( 1 ) );
  assert_eq!( book.best( other, Side::Buy ).unwrap().order.id, OrderId( 2 ) );

  // A third, never-mentioned instrument is simply empty — no separate
  // registration step exists to have skipped.
  let untouched = InstrumentId( 3 );
  assert!( book.side( untouched, Side::Buy ).next().is_none() );
  assert!( book.best( untouched, Side::Sell ).is_none() );

  // Cancelling by the wrong instrument finds nothing, even though the id
  // itself does rest — somewhere else. A lookup that searched globally by id
  // and ignored the instrument argument would wrongly remove order 2 here.
  assert!( book.cancel( INSTRUMENT, OrderId( 2 ) ).is_none(), "order 2 rests on `other`, not `INSTRUMENT`" );
  assert_eq!( book.side( other, Side::Buy ).map( | resting | resting.order.id.0 ).collect::< Vec< _ > >(), vec![ 2 ], "untouched by the mismatched cancel above" );

  // The global views see both instruments together.
  assert_eq!( book.len(), 2 );
  assert_eq!( book.iter().count(), 2 );

  // Cancelling by the right instrument works as normal.
  assert!( book.cancel( other, OrderId( 2 ) ).is_some() );
  assert_eq!( book.len(), 1 );
}

/// `account_rests` counts one account's orders across both sides of one
/// instrument, and nothing on another instrument.
#[ test ]
fn account_rests_counts_both_sides_of_one_instrument()
{
  let mine = | resting : Resting | Resting { order : Order { account : AccountId( 9 ), ..resting.order }, ..resting };
  let mut book = Book::new();
  assert!( book.insert( mine( rest( 1, Side::Buy, "1.00", 1, 1 ) ) ) );
  assert!( book.insert( mine( rest( 2, Side::Sell, "2.00", 1, 2 ) ) ) );
  assert!( book.insert( rest( 3, Side::Buy, "1.00", 1, 3 ) ) );
  assert!( book.insert( mine( rest_on( InstrumentId( 2 ), 4, Side::Buy, "1.00", 1, 4 ) ) ) );

  assert_eq!( book.account_rests( INSTRUMENT, AccountId( 9 ) ), 2 );
  assert_eq!( book.account_rests( INSTRUMENT, AccountId( 3 ) ), 1 );
  assert_eq!( book.account_rests( INSTRUMENT, AccountId( 7 ) ), 0 );
}
