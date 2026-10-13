//! Plain, copied rows of what rests on one instrument's book — a shape
//! another crate can persist or compare, independent of the live `Book` once
//! taken. Closes hard problems 16 (closed types) and 22 (snapshot of a book),
//! and feature 27 (book snapshot rows). The rows, not file format 012's
//! encoding of them.
//!
//! # Three pitfalls avoided by construction
//!
//! See [`docs/pitfall`](docs/pitfall).
//!
//! 1. **Aliasing the live book.** [`RestRow`] holds only `Copy` values taken
//!    out of each `Resting`, so a later cancel or fill cannot reach a taken
//!    [`BookSnap`].
//! 2. **A checksum riding hash-bucket order.** [`snap_take`] reads sorted
//!    levels through `Book::side`, so the rows come out in the same order
//!    every run.
//! 3. **A tick read from the clock.** [`snap_take`] takes `tick` from the
//!    caller; nothing here calls `Instant::now()`.
//!
//! # No `exchange_order` dependency, and no `SnapError`
//!
//! See [`docs/decisions`](docs/decisions).

use exact_arith::{ Money, Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ InstrumentId, OrderId };
use exchange_side::Side;

/// One resting order, flattened to plain, independently-owned values.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct RestRow
{
  /// The resting order's identity.
  pub order : OrderId,
  /// The price it rests at.
  pub price : Price,
  /// What remains of it, unfilled.
  pub qty : Quantity,
}

/// A point-in-time copy of one instrument's resting book.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct BookSnap
{
  /// Which instrument this snapshot is of.
  pub instrument : InstrumentId,
  /// The caller-supplied tick this snapshot was taken at — never read from a
  /// clock; see this crate's own module doc.
  pub tick : Money,
  /// Every resting order, bids then asks, each side in priority order.
  pub rows : Vec< RestRow >,
}

/// Copy every order resting on `instrument` into a [`BookSnap`], stamped with
/// the caller's `tick`.
///
/// Reads `instrument`'s two sides, not the whole-book `Book::iter`, so no
/// other instrument's orders get in.
#[ must_use ]
pub fn snap_take( book : &Book, instrument : InstrumentId, tick : Money ) -> BookSnap
{
  let to_row = | resting : &Resting | RestRow { order : resting.order.id, price : resting.order.price, qty : resting.remaining };
  let rows = book.side( instrument, Side::Buy ).map( to_row )
  .chain( book.side( instrument, Side::Sell ).map( to_row ) )
  .collect();

  BookSnap { instrument, tick, rows }
}

/// How many rows `snap` holds.
#[ must_use ]
pub fn snap_len( snap : &BookSnap ) -> usize
{
  snap.rows.len()
}
