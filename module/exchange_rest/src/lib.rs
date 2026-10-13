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
//! refusal. Nothing here matches, so a replacement that would trade is
//! refused too, rather than rested across the opposite side.

use exchange_book::{ Book, Resting };
use exchange_id::{ InstrumentId, OrderId };
use exchange_side::side_accepts;

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
/// The replacement's place is its own `arrival`'s. Passing the original's
/// keeps the original's place, which the family's amend rule allows only for
/// a smaller quantity at the same price
/// (`exchange_core/docs/algorithm/003_cancel_and_amend_semantics.md`).
///
/// # Errors
///
/// Checked in this order, each with nothing touched:
/// [`RestReplaceError::InstrumentMismatch`] if `new_resting` names another
/// instrument; [`RestReplaceError::Refused`] if [`Book::insert`] would refuse
/// it; [`RestReplaceError::WouldTake`] if its price reaches the best opposite
/// order, other than `old_id`; [`RestReplaceError::Missing`] if `old_id` does
/// not rest on `instrument`.
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

  // Fix(rest_replace_refuses_a_replacement_that_would_take):
  // Root cause: only `insert`'s checks ran, so a replacement priced into the
  // opposite side rested there untraded and the book was left crossed.
  // Pitfall: the original may sit on that side itself, under a replacement
  // that changes side; it is about to leave and crosses nothing.
  let side = new_resting.order.side;
  let would_take = book.side( instrument, side.opposite() )
    .find( | resting | resting.order.id != old_id )
    .is_some_and( | best | side_accepts( side, new_resting.order.price, best.order.price ) );
  if would_take
  {
    return Err( RestReplaceError::WouldTake );
  }

  let Some( old ) = book.cancel( instrument, old_id )
  else
  {
    return Err( RestReplaceError::Missing );
  };

  let seated = book.insert( new_resting );
  assert!( seated, "insertable() checked every refusal insert() makes, with old_id gone" );
  Ok( old )
}
