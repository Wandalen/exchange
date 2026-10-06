//! One order record — a resting order or a taker — with the account that
//! submitted it, the instrument it trades, its time-in-force, and the
//! submitter's own id for it.
//!
//! A single-tier dependent: `exchange_id` for `InstrumentId`/`OrderId`/
//! `AccountId`, `exchange_side` for `Side`, `exchange_tif` for `Tif`,
//! `exact_arith` for the decimal grid. This crate has no concept of a price
//! level or a book — that is `exchange_level`'s and `exchange_book`'s concern,
//! not this one's.
//!
//! `Order` moved here from `exchange_types`, gaining the two fields the real
//! struct was missing against the source design: `instrument` and `tif`.
//! `client` is an addition beyond the source design — the submitter's
//! `ClientOrderId`, which `exchange_core` uses to refuse a retry.
//!
//! # Not built: `order_new`/`order_qty_set`/`order_qty_left`/`order_is_empty`/`OrderError`
//!
//! The source design specifies a constructor and quantity-mutation helpers
//! plus a dedicated `OrderError`. The real design never adopted any of them:
//! every field here stays `pub`, so a plain struct literal is the one way to
//! build an `Order` and there is no second, fallible path to keep in sync with
//! it. Quantity never mutates on `Order` itself — `order.quantity` is what was
//! submitted, fixed for the order's lifetime — the shrinking remainder lives
//! on a separate wrapper (`exchange_book::Resting`, backed by
//! `exchange_level::LevelNode`) once the order rests, since only a resting
//! order has a remainder to track. And a zero-quantity order is refused at
//! `exchange_core`'s submission boundary via `RejectReason::ZeroQuantity`,
//! not at construction.
//!
//! # `Obligation` moved here too
//!
//! The dependency tree the family builds from never assigned `Obligation` a
//! crate of its own, and what an order commits is order-shaped, not a
//! standalone concept. The free function that computes it,
//! `exchange_types::obligation`, stays where it is, along with `notional`/
//! `TypeError` — `exchange_types` now depends forward on this crate for
//! `Order`/`Obligation` rather than the reverse, which is the shape every
//! crate `exchange_types` sheds types to ends up in.

use exact_arith::{ Price, Quantity };
use exchange_id::{ AccountId, ClientOrderId, InstrumentId, OrderId };
use exchange_side::Side;
use exchange_tif::Tif;

/// An amount of currency — a notional, a reservation, a balance. Distinct
/// from [`Price`] only in reading: the type is identical, but `Obligation::Cash`
/// holds a total, not a per-unit price.
pub type Amount = Price;

/// A submitted limit order.
///
/// Only limit orders exist in this slice. Market, stop, iceberg and post-only
/// are named as extensions in the family's own design documents and are not
/// implemented, because no behaviour here needs them yet.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct Order
{
  /// This order's identity.
  pub id : OrderId,
  /// The instrument it trades.
  pub instrument : InstrumentId,
  /// The participant who submitted it.
  pub account : AccountId,
  /// Which side of the book it stands on.
  pub side : Side,
  /// The worst price it will accept — a ceiling for a buy, a floor for a sell.
  pub price : Price,
  /// The quantity submitted, before any of it is filled.
  pub quantity : Quantity,
  /// How long its remainder may rest once a match pass is done.
  pub tif : Tif,
  /// The submitter's own id for it, if it gave one. `exchange_core` refuses a
  /// second order from the same account under the same one.
  pub client : Option< ClientOrderId >,
}

/// What an order commits until it fills or cancels.
///
/// Two shapes because two things are conserved: a buy owes currency it does
/// not yet know the final price of, and a sell owes the asset itself.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum Obligation
{
  /// Currency, reserved by a buy at its own limit price.
  Cash( Amount ),
  /// The asset, reserved by a sell.
  Asset( Quantity ),
}
