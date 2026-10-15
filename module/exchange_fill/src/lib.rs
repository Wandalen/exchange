//! The trade record and the event stream — the Contract's two outputs.
//!
//! [`Trade`] is the one record of a match. "Fill" survives only as a verb —
//! an order is fully or partially filled — never as a second record that
//! could disagree with the first. [`Event`] numbers every state change;
//! [`EventKind`] says which one it was.
//!
//! [`Trade::taker_side`] repeats what `exchange_escrow::settle` takes as a
//! parameter: `exchange_conserve` classifies a batch of trades with nothing
//! but the trades themselves.

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
  /// The executed-price rule: **a match executes at the resting order's
  /// price**, never at the aggressor's.
  ///
  /// The maker published its price and the taker crossed it deliberately.
  /// Executing at the taker's limit would hand the spread to whoever arrived
  /// second — a fee by another name. A function rather than a comment on the
  /// match loop, so the rule has one home and a test can name it.
  #[ must_use ]
  pub const fn executed_price( maker : Price, _taker : Price ) -> Price
  {
    maker
  }
}

/// Why an order was refused.
///
/// A closed set rather than a string, so a caller branches on the reason
/// instead of parsing prose. A new reason is a new variant, and every
/// exhaustive match downstream has to name it.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum RejectReason
{
  /// The order's quantity was zero. A zero-quantity order can neither fill
  /// nor rest meaningfully.
  ZeroQuantity,
  /// The order's price was below zero.
  ///
  /// `Price` is signed. A negative buy notional would pay the account for
  /// committing, and a negative sell would settle inside out — value from
  /// nothing either way. Zero is accepted: it moves the asset for no
  /// currency, which is odd but conserves.
  NegativePrice,
  /// The account is not known to the exchange.
  UnknownAccount,
  /// The account holds less than the order commits.
  InsufficientFunds,
  /// The order's own obligation is not expressible — its notional exceeds the
  /// representable ceiling, or does not land exactly on the scale. See
  /// `notional` in `exchange_types`.
  ObligationUnrepresentable,
  /// The account's funds were sufficient, but adding this order to what it
  /// already has reserved would pass the representable ceiling.
  ///
  /// Distinct from [`RejectReason::InsufficientFunds`]: nothing is missing.
  /// Reachable when an account with a large reservation earns fresh funds —
  /// settling as a seller, say — and places another order.
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
  /// It would have crossed an order from the same account, and the
  /// self-match policy withdrew it — see `exchange_stp::SelfMatchPolicy`.
  SelfMatch,
  /// The order's time-in-force forbade resting the remainder — IOC dropped
  /// what a partial fill left, or FOK could not fill in full and traded
  /// nothing.
  TimeInForce,
}

/// One entry in the event stream.
///
/// Every state change emits exactly one, and nothing changes without one.
/// For an [`EventKind::Trade`] the common fields name the **aggressor**; the
/// resting side travels in the payload.
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
/// No `OrderFilled` kind: an order is filled when its trades' quantities sum
/// to what it submitted, which the [`EventKind::Trade`] events already say.
/// A terminal event would be a second record of one fact.
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
