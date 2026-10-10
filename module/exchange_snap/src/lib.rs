//! Plain, copied rows of what rests on a book — a shape another crate can
//! persist or compare, independent of the live `Book` the moment it's taken.
//!
//! Without this crate, the market dies the moment the process saves and
//! restarts: `exchange_book::Book::iter()` can walk the live book, but
//! nothing here can be serialized or diffed without reaching back into it.
//! Closes hard problems 16 (closed types) and 22 (snapshot of a book), and
//! feature 27 (book snapshot rows). Not file format 012 — this crate gives
//! the row shape, not a save-file encoding.
//!
//! # Three pitfalls this design avoids by construction
//!
//! See [`docs/pitfall`](docs/pitfall) for the full write-ups; summarized
//! here because each one shaped a specific choice below:
//!
//! 1. **Aliasing the live book.** [`RestRow`] holds only `Copy` values
//!    (`OrderId`, `Price`, `Quantity`) taken *out of* each `Resting`, never a
//!    reference into `Book`'s own storage — a cancel or fill after
//!    [`snap_take`] can never reach back into an already-taken [`BookSnap`].
//! 2. **A checksum riding hash-bucket order.** `Book`'s own representation
//!    is sorted levels per side, never a `HashMap`; [`snap_take`] reads it via
//!    `Book::side()`, so `BookSnap::rows` inherits the same hash-free,
//!    deterministic order. A checksum built over it later is sound on this
//!    count without that later code having to re-establish it.
//! 3. **A tick read from the clock.** [`snap_take`] takes `tick` as a
//!    parameter, supplied by the caller — nothing here calls `Instant::now()`
//!    or any other live clock.
//!
//! # No `exchange_order` dependency, and no `SnapError`
//!
//! See [`docs/decisions`](docs/decisions) for both: the source design names
//! `exchange_order` as a dependency and `SnapError { Full }` as part of the
//! exposed surface, and this crate takes neither.

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
  /// Every resting order, bids then asks, each side still in priority order
  /// (the same sequence [`exchange_book::Book::iter`] publishes).
  pub rows : Vec< RestRow >,
}

/// Copy every resting order on `book` for `instrument` into a [`BookSnap`],
/// stamped with the caller-supplied `tick`.
///
/// Reads `instrument`'s own two sides specifically — `Book::side`, never the
/// whole-book `Book::iter` — so a snapshot of one instrument never picks up
/// another instrument's resting orders now that one `Book` holds several.
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
