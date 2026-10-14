//! The three non-matching ways an order moves on the book: rest, cancel,
//! replace.
//!
//! [`rest_place`] and [`rest_cancel`] forward to [`Book::insert`] and
//! [`Book::cancel`]. Idempotency, caps, halt and escrow stay with the caller
//! — `exchange_core` checks them before it rests through `rest_place`.
//!
//! [`rest_replace`] is the one operation of its own. As a cancel followed by
//! a place, a refused replacement would leave neither order resting, and
//! putting the original back would seat it behind every order sharing its
//! arrival. So it checks the replacement first and touches nothing on a
//! refusal. A replacement must be the same order, amended — same side,
//! account, time in force and client id, arriving as the amend rule allows —
//! and, since nothing here matches, must not reach the opposite side.

use exchange_book::{ Book, Resting };
use exchange_id::{ InstrumentId, OrderId };
use exchange_side::{ Side, side_accepts };

/// Place `resting` on `book` — [`Book::insert`].
///
/// Returns `false` on the same terms: a `remaining` of zero or above
/// `order.quantity`, or an `order.id` already resting.
#[ must_use ]
pub fn rest_place( book : &mut Book, resting : Resting ) -> bool
{
  book.insert( resting )
}

/// Withdraw the order `id` from `instrument`'s book, either side —
/// [`Book::cancel`].
///
/// [`None`] if it does not rest: it may have filled or been cancelled
/// already, a race rather than a caller error.
pub fn rest_cancel( book : &mut Book, instrument : InstrumentId, id : OrderId ) -> Option< Resting >
{
  book.cancel( instrument, id )
}

/// Why [`rest_replace`] could not place the replacement.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum RestReplaceError
{
  /// `new_resting.order.instrument` is not `instrument`.
  InstrumentMismatch,
  /// `old_id` does not rest on `instrument`.
  Missing,
  /// The replacement has another side, account, time in force or client id
  /// than the order it replaces: that is a cancel and a new order, not an
  /// amend.
  NotAnAmend,
  /// The replacement's `arrival` ranks it ahead of where the amend rule puts
  /// it: a larger quantity or another price must arrive after the original,
  /// a smaller or equal quantity at the same price no earlier.
  JumpsQueue,
  /// [`Book::insert`] would refuse the replacement: a `remaining` of zero or
  /// above its order's quantity, or an `order.id` resting elsewhere.
  Refused,
  /// The replacement's price reaches the best order on the opposite side.
  /// Resting it would leave the book crossed; it has to be matched instead.
  WouldTake,
}

impl core::fmt::Display for RestReplaceError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::InstrumentMismatch => write!( f, "new_resting's own instrument disagrees with the instrument argument" ),
      Self::Missing => write!( f, "no order with that id is resting on this instrument" ),
      Self::NotAnAmend => write!( f, "the replacement changes the order's side, account, time in force or client id" ),
      Self::JumpsQueue => write!( f, "the replacement's arrival ranks it ahead of where the amend rule allows; the original order is unchanged" ),
      Self::Refused => write!( f, "the replacement was refused; the original order is unchanged" ),
      Self::WouldTake => write!( f, "the replacement would trade against the opposite side; the original order is unchanged" ),
    }
  }
}

impl core::error::Error for RestReplaceError {}

/// Replace the order `old_id` resting on `instrument` with `new_resting`, in
/// one step, and return the order replaced.
///
/// Asks [`Book::insertable`] first, counting `old_id` as gone, then cancels
/// `old_id` and inserts `new_resting`. A refusal touches nothing, so the
/// original keeps its place in the queue.
///
/// The replacement's place is its own `arrival`'s. The family's amend rule
/// (`exchange_core/docs/algorithm/003_cancel_and_amend_semantics.md`) lets a
/// smaller or equal quantity at the same price keep the original's arrival;
/// anything else must arrive after it. Which later arrival is the caller's:
/// only it stamps them.
///
/// # Errors
///
/// Checked in this order, each with nothing touched:
/// [`RestReplaceError::InstrumentMismatch`] if `new_resting` names another
/// instrument; [`RestReplaceError::Refused`] if [`Book::insert`] would refuse
/// it; [`RestReplaceError::Missing`] if `old_id` does not rest on
/// `instrument`; [`RestReplaceError::NotAnAmend`] if it changes the order's
/// side, account, time in force or client id;
/// [`RestReplaceError::JumpsQueue`] if its arrival breaks the amend rule;
/// [`RestReplaceError::WouldTake`] if its price reaches the best opposite
/// order.
///
/// ```rust
/// use exact_arith::{ Price, Quantity };
/// use exchange_book::{ Book, Resting };
/// use exchange_id::{ AccountId, InstrumentId, OrderId };
/// use exchange_order::Order;
/// use exchange_rest::rest_replace;
/// use exchange_seq::Sequence;
/// use exchange_side::Side;
/// use exchange_tif::Tif;
///
/// let order = | id, price : &str, quantity | Order
/// {
///   id : OrderId( id ), instrument : InstrumentId( 1 ), account : AccountId( id ),
///   side : Side::Sell, price : Price::parse( price ).unwrap(),
///   quantity : Quantity::from_int( quantity ).unwrap(), tif : Tif::Gtc, client : None,
/// };
///
/// let mut book = Book::new();
/// let original = order( 1, "2.50", 4 );
/// book.insert( Resting { order : original, remaining : original.quantity, arrival : Sequence( 1 ) } );
///
/// let replacement = order( 1, "2.60", 6 );
/// let resting = Resting { order : replacement, remaining : replacement.quantity, arrival : Sequence( 2 ) };
/// let old = rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), resting ).unwrap();
///
/// assert_eq!( old.order.price, Price::parse( "2.50" ).unwrap() );
/// assert_eq!( book.best( InstrumentId( 1 ), Side::Sell ).unwrap().order.price, Price::parse( "2.60" ).unwrap() );
/// ```
pub fn rest_replace( book : &mut Book, instrument : InstrumentId, old_id : OrderId, new_resting : Resting ) -> Result< Resting, RestReplaceError >
{
  // Fix(rest_replace_refuses_a_new_resting_for_a_different_instrument):
  // Root cause: `instrument` was read only for the cancel; `insert` seats the
  // replacement on its own order's instrument, and the two were never compared.
  // Pitfall: when an argument and the payload name the same thing, check they
  // agree before either drives a write.
  if new_resting.order.instrument != instrument
  {
    return Err( RestReplaceError::InstrumentMismatch );
  }

  // Fix(a_refused_replace_sent_the_original_behind_its_peers):
  // Root cause: the original was cancelled first and reinserted on refusal.
  // `insert` seats an order behind every peer sharing its arrival, so it came
  // back behind those it had been ahead of.
  // Pitfall: reinserting by the same key is not putting back where it was.
  if !book.insertable( &new_resting, Some( old_id ) )
  {
    return Err( RestReplaceError::Refused );
  }

  let Some( old ) = book.side( instrument, Side::Buy ).chain( book.side( instrument, Side::Sell ) )
    .find( | resting | resting.order.id == old_id )
    .copied()
  else
  {
    return Err( RestReplaceError::Missing );
  };

  // Fix(rest_replace_refuses_a_replacement_that_is_another_order):
  // Root cause: nothing compared the replacement with the order it replaces,
  // so a replace could flip its side, hand it to another account, or turn it
  // into an IOC left resting.
  // Pitfall: a replace may rename the order; it may not make it another one.
  //
  // Fix(rest_replace_refuses_a_replacement_under_another_client_id):
  // Root cause: the client id was not compared, so a replace could move the
  // order to a client id its caller's idempotency record never claimed.
  // Pitfall: the order id is the book's and may change; the client id is the
  // owner's handle on the order and may not.
  let ( was, new ) = ( old.order, new_resting.order );
  if ( new.side, new.account, new.tif, new.client ) != ( was.side, was.account, was.tif, was.client )
  {
    return Err( RestReplaceError::NotAnAmend );
  }

  // Fix(rest_replace_refuses_a_replacement_that_jumps_the_queue):
  // Root cause: the replacement's arrival was taken as given, so one that
  // raised its quantity or moved its price kept the original's place by
  // carrying its arrival.
  // Pitfall: which later arrival is right is the stamper's to know; what
  // holds for any amend is that it arrives after the order it amends.
  let keeps_place = new.price == was.price && new_resting.remaining <= old.remaining;
  let jumps = if keeps_place { new_resting.arrival < old.arrival } else { new_resting.arrival <= old.arrival };
  if jumps
  {
    return Err( RestReplaceError::JumpsQueue );
  }

  // Fix(rest_replace_refuses_a_replacement_that_would_take):
  // Root cause: only `insert`'s checks ran, so a replacement priced into the
  // opposite side rested there untraded and the book was left crossed.
  // Pitfall: the opposite side counts every account, the replacing one too.
  let would_take = book.best( instrument, new.side.opposite() )
    .is_some_and( | best | side_accepts( new.side, new.price, best.order.price ) );
  if would_take
  {
    return Err( RestReplaceError::WouldTake );
  }

  book.cancel( instrument, old_id ).expect( "found resting above, and nothing moved since" );
  let seated = book.insert( new_resting );
  assert!( seated, "insertable() checked every refusal insert() makes, with old_id gone" );
  Ok( old )
}
