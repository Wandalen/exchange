//! One order record — a resting order or a taker — with the account that
//! submitted it, the instrument it trades, its time-in-force, and the
//! submitter's own id for it.
//!
//! No concept of a price level or a book — that is `exchange_level`'s and
//! `exchange_book`'s concern. Closes feature 3.
//!
//! # Not built: a constructor, quantity mutation, or `OrderError`
//!
//! Every field is `pub`, so a struct literal is the one way to build an
//! `Order`. `quantity` is what was submitted and never changes; a resting
//! remainder lives on `exchange_book::Resting`. A zero quantity is refused at
//! `exchange_core`'s submission boundary, not here — see
//! `docs/decisions/001_no_order_mutation_or_error.md`.
//!
//! # `Obligation` lives here too
//!
//! What an order commits is order-shaped, and the family's dependency tree
//! never gave it a crate of its own. The function computing it,
//! `exchange_types::obligation`, stays in `exchange_types`.

use exact_arith::{ Money, Price, Quantity };
use exchange_id::{ AccountId, ClientOrderId, InstrumentId, OrderId };
use exchange_side::Side;
use exchange_tif::Tif;

/// An amount of currency — a notional, a reservation, a balance. A total,
/// not a per-unit [`Price`].
pub type Amount = Money;

/// A submitted limit order.
///
/// Only limit orders exist. Post-only is a [`Tif`], not an order type; market,
/// stop and iceberg orders are not built.
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
