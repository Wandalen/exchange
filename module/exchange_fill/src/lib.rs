//! The trade record and the event stream — the Contract's two outputs.
//!
//! # Naming — `Trade`, not `Fill`
//!
//! The family's Contract says *trades out*; the matching algorithm talks
//! about *fills*. They are one thing, so there is one type: [`Trade`], the
//! record of one match. "Fill" survives as this crate's own name and as a
//! verb — an order is *fully filled* or *partially filled* — describing what
//! a Trade did to an order, never a second record of it. Two types here
//! would be two sources of truth for one event, and they would disagree
//! exactly when something had already gone wrong.
//!
//! # Extraction
//!
//! `Trade`, `Event`, `EventKind`, `RejectReason` and `CancelCause` moved here
//! from `exchange_types`, which re-exports all five unchanged — the same
//! compatibility convention every earlier extraction in this family has
//! used. `CancelCause` moved alongside `EventKind` rather than staying
//! behind: `EventKind::OrderCancelled` holds one, so leaving it in
//! `exchange_types` while `EventKind` left would have forced a circular
//! dependency back up to the crate this one is extracted from — the same
//! "move the whole self-contained cluster together" reasoning
//! `exchange_order` already documents for `Order`/`Obligation`/`TypeError`.
//!
//! `notional`/`TypeError`/`obligation` are **not** here — they stayed in
//! `exchange_types` when `Order`/`Obligation` moved to `exchange_order`, and
//! nothing in this crate's own cluster (`Trade`/`Event`/`EventKind`/
//! `RejectReason`/`CancelCause`) needs them. `RejectReason::ObligationUnrepresentable`'s
//! own doc comment mentions `notional` in prose rather than as an intra-doc
//! link for exactly this reason: this crate does not depend on
//! `exchange_types`, so a bracketed link to it could never resolve.
//!
//! # `taker_side`, added to `Trade`
//!
//! The real struct lacked a field recording which side the taker was on.
//! `exchange_escrow::settle` already takes `taker_side` as its own separate
//! parameter, computed by the caller from the original submission context —
//! so this field is redundant with that parameter, not a new requirement.
//! It is added anyway, on `Trade` itself, because `exchange_conserve`
//! (built from this crate, not from `exchange_escrow`) needs to classify a
//! *batch* of trades by taker side with no submission context available at
//! all — only the trades themselves. `exchange_escrow::settle`'s own
//! signature is left untouched: decision 2 of this family's refactor plan
//! extracts `exchange_escrow` as-is, and collapsing the redundancy would be
//! an API change to a crate that stage explicitly does not touch.

use exact_arith::{ Price, Quantity };
use exchange_id::{ AccountId, OrderId };
use exchange_order::Obligation;
use exchange_seq::Sequence;
use exchange_side::Side;

/// One match: the record of quantity changing hands between two orders.
///
/// Both sides travel in the record. The aggressor is the incoming order that
/// caused the match; the maker is the order that was already resting. The
/// classification is a property of the trade rather than something each
/// consumer re-derives — and re-derives differently.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct Trade
{
  /// The incoming order that caused the match.
  pub taker : OrderId,
  /// The aggressor's account.
  pub taker_account : AccountId,
  /// Which side the taker was on. The maker was the other side, always —
  /// a trade only ever has the two.
  pub taker_side : Side,
  /// The resting order it matched against.
  pub maker : OrderId,
  /// The resting order's account.
  pub maker_account : AccountId,
  /// The price it executed at — the maker's, per [`Trade::executed_price`].
  pub price : Price,
  /// How much changed hands.
  pub quantity : Quantity,
}

impl Trade
{
  /// The executed-price rule this family applies: **a match executes at
  /// the resting order's price**, never at the aggressor's.
  ///
  /// The family's matching algorithm lists this as open and leaves it to
  /// the implementation; this is where it is closed. The reason is that the
  /// resting order's price is the only one both parties had a chance to see:
  /// the maker published it and the taker crossed it deliberately. Executing
  /// at the taker's limit instead would quietly hand the entire spread to
  /// whoever arrived second, which is a fee by another name and one the fee
  /// schedule could not account for.
  ///
  /// Stated as a documented function rather than a comment on the match loop
  /// so that the rule has one home and a test can name it.
  #[ must_use ]
  pub const fn executed_price( maker : Price, _taker : Price ) -> Price
  {
    maker
  }
}

/// Why an order was refused.
///
/// A closed set rather than a string. Which reasons exist will grow; that a
/// caller can branch on the reason rather than parse prose is the part that
/// cannot be added later without every existing caller's error handling being
/// wrong.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum RejectReason
{
  /// The order's quantity was zero. A zero-quantity order can neither fill
  /// nor rest meaningfully.
  ZeroQuantity,
  /// The order's price was below zero.
  ///
  /// `Price` is signed, and nothing in the type refuses a negative — so the
  /// refusal is here. A negative price makes a buy's notional negative,
  /// which is a commitment that pays the account rather than costing it; and
  /// it lets a sell rest at a price that turns its eventual settlement
  /// inside out. Both are value created from nothing.
  ///
  /// Zero is deliberately still accepted. A gift at zero moves asset for no
  /// currency, which is odd but conserves exactly — both sides consented and
  /// nothing appears. Only strictly-below-zero is refused, because only
  /// strictly-below-zero mints.
  NegativePrice,
  /// The account is not known to the exchange.
  UnknownAccount,
  /// The account holds less than the order commits.
  InsufficientFunds,
  /// The order's own obligation is not expressible — its notional exceeds the
  /// representable ceiling, or does not land exactly on the scale. See
  /// `notional` in `exchange_types`.
  ObligationUnrepresentable,
  /// The account's funds were sufficient, but committing them would push its
  /// *cumulative reservation* — the sum already promised to its other live
  /// orders, plus this one — past the representable ceiling.
  ///
  /// Distinct from [`RejectReason::InsufficientFunds`]: nothing is missing.
  /// Reachable when an account already carries a large reservation from an
  /// earlier order and then earns fresh, genuinely spendable funds — settling
  /// as a seller, say — before placing another: `available` comfortably
  /// covers the new order, but folding it into `reserved` alongside the
  /// existing commitment would not fit the type's declared ceiling.
  ReservationUnrepresentable,
  /// The order's instrument is currently halted — see `exchange_halt`.
  /// Resting orders are untouched; only a new arrival is refused.
  Halted,
  /// Resting this order would push its price level past its configured
  /// `max_rests`. Only reachable for an instrument with caps registered —
  /// see `exchange_cap`.
  RestsFull,
  /// Resting this order would open a new price level past its side's
  /// configured `max_levels`. Only reachable for an instrument with caps
  /// registered — see `exchange_cap`.
  LevelsFull,
  /// A post-only order would have taken liquidity on arrival — see
  /// `exchange_tif::Tif::PostOnly`.
  PostOnlyWouldTake,
  /// The account already placed an order under this `ClientOrderId` — a
  /// retry, not a new order.
  DuplicateClientId,
  /// Resting this order would push its account past the configured
  /// `max_account_rests`. Only reachable for an instrument with caps
  /// registered — see `exchange_cap`.
  AccountFull,
  /// The order's price is not a multiple of its instrument's tick — see
  /// `exchange_spec::price_fits`.
  PriceOffTick,
  /// The order's quantity is not a multiple of its instrument's lot — see
  /// `exchange_spec::qty_fits`.
  QuantityOffLot,
  /// The order's instrument has no registered spec.
  UnknownInstrument,
}

/// Why a remainder was withdrawn.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum CancelCause
{
  /// The owner asked for it.
  Request,
  /// The engine withdrew it under the self-match policy configured for the
  /// book, because it would otherwise have crossed an order sharing the same
  /// self-match key — see `SelfMatchPolicy` in the matching engine.
  SelfMatch,
  /// The order's own Time-in-Force forbade resting an unfilled remainder —
  /// IOC dropped what was left after a partial fill, or FOK found it could
  /// not be filled in full and the whole order came back untraded. Added
  /// once `exchange_core::Exchange::step_place` actually disposed of a
  /// remainder this way instead of only `exchange_match::cross` reporting
  /// one — see that method's own `Fix(tif_dropped_remainder_leaked_its_own_reservation)`
  /// comment for why a cause variant and a release both had to exist
  /// together, not just one of them.
  TimeInForce,
}

/// One entry in the event stream — the Contract's second output, beside the
/// trades themselves.
///
/// Every state change emits exactly one of these, and no state changes without
/// one. Four fields are common to every kind, and for a [`EventKind::Trade`]
/// they name the **aggressor**; the resting side travels in the payload.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct Event
{
  /// Position in the total order. Numbers every event, increasing by one, so
  /// a consumer detects loss by arithmetic alone.
  pub sequence : Sequence,
  /// The order this event concerns.
  pub order : OrderId,
  /// The account owning `order` — attribution without a lookup.
  pub account : AccountId,
  /// What happened.
  pub kind : EventKind,
}

/// What an [`Event`] records.
///
/// There is deliberately no `OrderFilled` kind: an order is filled when its
/// trades' quantities sum to its submitted quantity, a fact the
/// [`EventKind::Trade`] events already carry exactly. A dedicated terminal
/// event would be a second record of one fact, and a consumer that trusted it
/// would diverge from one that computed it — silently, in exactly the cases
/// where the two disagree.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum EventKind
{
  /// Validation and reservation both succeeded, before any matching. This
  /// event's `sequence` is the order's arrival position.
  OrderAccepted
  {
    /// The side it stands on.
    side : Side,
    /// Its limit price.
    price : Price,
    /// The quantity submitted.
    quantity : Quantity,
    /// What was moved from available to reserved.
    reserved : Obligation,
  },
  /// Validation failed, or the reservation could not be taken in full. No
  /// book state ever existed and no units were reserved.
  OrderRejected
  {
    /// Why.
    reason : RejectReason,
  },
  /// The match loop generated one trade.
  Trade( Trade ),
  /// A remainder was withdrawn and its reservation returned.
  OrderCancelled
  {
    /// Why.
    cause : CancelCause,
    /// How much was withdrawn.
    quantity : Quantity,
    /// What returned from reserved to available.
    released : Obligation,
  },
}
