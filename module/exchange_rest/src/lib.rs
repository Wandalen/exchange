//! The three non-matching ways an order moves on the book: rest, cancel,
//! replace.
//!
//! # Thin, not the proposal's full orchestration
//!
//! `docs/exposed_item/015_exchange_rest_items.md` found `rest_place`'s and
//! `rest_cancel`'s real equivalents already spread across `exchange_core`'s
//! own `submit`/`cancel` — idempotency, cap, and escrow checks included, not
//! just the book-level move. This crate does **not** consolidate that
//! orchestration out of `exchange_core`: the family's own Stage 9 plan keeps
//! `Exchange::cancel` unchanged on purpose, which only holds if this crate
//! stays a thin wrapper over [`exchange_book::Book`] rather than the thing
//! `exchange_core` delegates to. So `exchange_core` keeps deciding
//! idempotency/cap/escrow inline, unchanged, and this crate owns only the
//! book-level primitive underneath that decision — the same skeleton-first
//! shape already used for `exchange_cap` (Stage 1) and `exchange_conserve`
//! (Stage 5): real, tested, standalone, not yet consumed by the facade.
//!
//! # `rest_replace` is new, not extracted
//!
//! `docs/exposed_item/015_exchange_rest_items.md` confirms no atomic
//! cancel-and-reinsert exists anywhere today — a caller wanting to replace
//! an order currently calls `cancel` then `submit` as two separate,
//! non-atomic steps, which can lose the old order's priority slot to a
//! third party arriving between them with nothing left to roll back to if
//! the second step fails. [`rest_replace`] closes that: cancel first, then
//! insert, and if the insert is refused, the original goes back exactly as
//! it was rather than staying cancelled with nothing to show for it.

use exchange_book::{ Book, Resting };
use exchange_id::{ InstrumentId, OrderId };

/// Place `resting` on `book`.
///
/// A direct call-through to [`Book::insert`] today — the crate's own value
/// is in being the one place a caller reaches for *any* of the three
/// non-matching moves, not in adding logic `insert` does not already have.
///
/// Returns `false` on the same terms `insert` does: a zero-remaining
/// `resting`, or an `order.id` already resting on either side.
#[ must_use ]
pub fn rest_place( book : &mut Book, resting : Resting ) -> bool
{
  book.insert( resting )
}

/// Withdraw the order `id` resting on `instrument`'s book, whichever side it
/// is on.
///
/// A direct call-through to [`Book::cancel`] — see [`rest_place`]'s doc for
/// why this crate adds no logic of its own here.
///
/// Returns [`None`] if no such order rests — a race result, not a caller
/// error (see [`Book::cancel`]'s own doc comment): the order may already
/// have filled, been cancelled, or `instrument` may never have had an order
/// rest on it at all.
pub fn rest_cancel( book : &mut Book, instrument : InstrumentId, id : OrderId ) -> Option< Resting >
{
  book.cancel( instrument, id )
}

/// Why [`rest_replace`] could not place the replacement.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum RestReplaceError
{
  /// `new_resting.order.instrument` disagreed with `instrument` — nothing
  /// was touched; the order named by `old_id` is still resting exactly as
  /// it was.
  InstrumentMismatch,
  /// `old_id` was not resting on `instrument` — nothing to replace.
  Missing,
  /// The replacement was refused by [`Book::insert`]: a zero-remaining
  /// `Resting`, or an `order.id` already resting elsewhere on the book. The
  /// order named by `old_id` has been put back exactly as it was.
  Refused,
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
    }
  }
}

impl core::error::Error for RestReplaceError {}

/// Atomically replace the order resting at `old_id` on `instrument` with
/// `new_resting`.
///
/// Cancels `old_id` first, then inserts `new_resting`. If that insert is
/// refused, the cancelled order is reinserted unchanged before returning —
/// a caller never observes a state where the old order is gone and the new
/// one never arrived. See the module doc's "`rest_replace` is new, not
/// extracted" section for why this exists at all.
///
/// Returns the original `Resting` that was replaced, on success.
///
/// # Errors
///
/// [`RestReplaceError::InstrumentMismatch`] if `new_resting.order.instrument`
/// disagrees with `instrument` — checked first, before anything is touched.
/// [`RestReplaceError::Missing`] if `old_id` is not resting on `instrument`.
/// [`RestReplaceError::Refused`] if `new_resting` is refused by
/// [`Book::insert`] — the original stays exactly as it was.
///
/// ```rust
/// use exact_arith::{ Money, Quantity };
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
///   side : Side::Sell, price : Money::parse( price ).unwrap(),
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
/// assert_eq!( old.order.price, Money::parse( "2.50" ).unwrap() );
/// assert_eq!( book.best( InstrumentId( 1 ), Side::Sell ).unwrap().order.price, Money::parse( "2.60" ).unwrap() );
/// ```
pub fn rest_replace( book : &mut Book, instrument : InstrumentId, old_id : OrderId, new_resting : Resting ) -> Result< Resting, RestReplaceError >
{
  // Fix(BUG-001): new_resting's own order.instrument was never checked against the
  // instrument argument before being handed to insert(), which has no instrument
  // parameter of its own to catch the disagreement.
  // Root cause: `instrument` was read only for the cancel() lookup; insert() derives
  // its target book purely from new_resting.order.instrument, so the two values were
  // never compared anywhere in the function.
  // Pitfall: when a selector parameter and a payload's own embedded field encode the
  // same identity, trusting only one of them lets a caller act on a different target
  // than the one the lookup already confirmed — assert they agree before either is used.
  if new_resting.order.instrument != instrument
  {
    return Err( RestReplaceError::InstrumentMismatch );
  }

  let Some( old ) = book.cancel( instrument, old_id )
  else
  {
    return Err( RestReplaceError::Missing );
  };

  if book.insert( new_resting )
  {
    Ok( old )
  }
  else
  {
    let restored = book.insert( old );
    debug_assert!( restored, "an id just freed by cancel(), with its own prior-valid remaining, cannot be refused by insert()" );
    Err( RestReplaceError::Refused )
  }
}
