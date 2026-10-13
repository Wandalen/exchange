//! The crossing engine: one incoming order against a book, producing trades.
//!
//! Three outcomes, none of them an error:
//!
//! - **Full fill** — the incoming quantity is consumed.
//! - **Partial fill** — some trades, and a remainder for the caller.
//! - **No cross** — no trades. Nothing on the book was at an acceptable
//!   price, or nothing it could be paid for exactly.
//!
//! The third is the one worth naming: an engine that always finds a match
//! passes every matching test and is wrong. Declining is behaviour.
//!
//! # A fill declined at a price that crosses
//!
//! [`exchange_types::notional`] refuses to round a price times a quantity,
//! and settlement needs the amount paid exactly. A partial fill can need a
//! seventh decimal even when both orders were accepted, and by the time
//! settlement fails, [`Book::consume_best`] has taken the resting order off
//! the book for good. So the loop checks first and declines such a fill
//! instead of generating it.
//!
//! It stops at that order rather than skip past it to a worse price. The
//! cost: a taker can rest across it, and a remainder no fill can pay — 0.001
//! at 1.0001 — stops every taker on its side until its owner cancels it. A
//! smaller fill might have been payable, but choosing one needs a minimum
//! fill increment derived from the price, which is not decided. An
//! instrument whose tick times lot is exact at the money scale never meets
//! any of this.
//!
//! # Self-match prevention
//!
//! A pair sharing one account never trades. The check runs before the pair's
//! [`Trade`] is built, and before the settleability check. [`SelfMatchPolicy`]
//! (`exchange_stp`'s, re-exported) says which side is withdrawn; the caller
//! passes it on every call, since this crate keeps no state.
//!
//! # Time-in-force
//!
//! `cross` never rests the incoming order, so GTC and IOC differ only in what
//! the caller does with the remainder. A FOK order is probed against a clone
//! of the book, and run on the real book only if the probe fills it whole;
//! otherwise it comes back as a no-cross with the book untouched. A
//! post-only order that [`would_take`] is refused with
//! [`MatchError::PostOnlyWouldTake`], also untouched — `exchange_core` checks
//! `would_take` first so the refusal is on record.
//!
//! # What this crate does not decide
//!
//! Who is next — [`exchange_book`] hands out a side best first. The price —
//! [`Trade::executed_price`]. Funds — escrow settles the trades this returns.
//!
//! # Determinism
//!
//! No clock, no hash iteration, no address comparison: the same book and
//! incoming order give the same trades, so a fill can be re-derived from the
//! record.

use exact_arith::{ KindError, Price, Quantity };
use exchange_book::Book;
use exchange_conserve::{ conserve_assert, ConserveError };
use exchange_fill::Trade;
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_side::side_accepts;
use exchange_tif::{ tif_requires_full, tif_takes };
use exchange_types::notional;

pub use exchange_stp::SelfMatchPolicy;

/// What crossing an order against a book produced.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct Crossing
{
  /// The trades generated, in the order they were generated.
  pub trades : Vec< Trade >,
  /// What is left of the incoming order; zero on a full fill.
  ///
  /// Counts a self-match-cancelled remainder too, so `filled() + remaining`
  /// is always the submitted quantity. If `cancelled` names the incoming
  /// order, this was withdrawn, not left for the caller to rest.
  pub remaining : Quantity,
  /// Every order self-match prevention withdrew, in the order it happened.
  /// Each says where it fell among `trades`; [`Crossing::steps`] interleaves
  /// the two.
  pub cancelled : Vec< SelfMatchCancellation >,
}

impl Crossing
{
  /// Whether the incoming order was entirely consumed.
  #[ must_use ]
  pub fn is_complete( &self ) -> bool
  {
    self.remaining == Quantity::ZERO
  }

  /// The total quantity that changed hands.
  ///
  /// # Errors
  ///
  /// [`KindError`] if the quantities do not sum within the type —
  /// unreachable for a `Crossing` built here, whose trades split one
  /// quantity.
  pub fn filled( &self ) -> Result< Quantity, KindError >
  {
    self.trades.iter().try_fold( Quantity::ZERO, | sum, trade | sum.checked_add( trade.quantity ) )
  }

  /// Every trade and self-match withdrawal, in the order `cross` made them —
  /// the order to settle, release and record them in.
  #[ must_use ]
  pub fn steps( &self ) -> Vec< CrossingStep< '_ > >
  {
    let mut steps = Vec::with_capacity( self.trades.len() + self.cancelled.len() );
    let mut cancelled = self.cancelled.iter().peekable();
    for ( done, trade ) in self.trades.iter().enumerate()
    {
      while let Some( withdrawn ) = cancelled.next_if( | withdrawn | withdrawn.trades_before <= done )
      {
        steps.push( CrossingStep::Cancelled( withdrawn ) );
      }
      steps.push( CrossingStep::Trade( trade ) );
    }
    steps.extend( cancelled.map( CrossingStep::Cancelled ) );
    steps
  }
}

/// One thing [`cross`] did — see [`Crossing::steps`].
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum CrossingStep< 'a >
{
  /// A trade.
  Trade( &'a Trade ),
  /// A self-match withdrawal.
  Cancelled( &'a SelfMatchCancellation ),
}

/// One order [`SelfMatchPolicy`] withdrew instead of letting it trade.
///
/// Reported, not acted on: the caller releases its reservation and records
/// the cancel, with cause `exchange_fill::CancelCause::SelfMatch`.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct SelfMatchCancellation
{
  /// The order withdrawn — either side of the self-matching pair.
  pub order : OrderId,
  /// The account both sides of the pair shared.
  pub account : AccountId,
  /// How much of it was withdrawn.
  pub quantity : Quantity,
  /// How many of the crossing's trades came before it. Cancel-resting
  /// withdraws an order and then trades behind it, so a withdrawal can
  /// precede trades.
  pub trades_before : usize,
}

/// Something the matching loop could not do.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum MatchError
{
  /// A quantity step failed. Unreachable — every quantity here splits the
  /// incoming one — and returned rather than unwrapped, since a panic
  /// mid-match leaves a book no event describes.
  Quantity( KindError ),
  /// The book refused a reduction or a self-match withdrawal the loop had
  /// decided on: the two disagree about what rests.
  BookDesynchronized,
  /// `exchange_conserve::conserve_assert` refused this call's trades.
  /// Unreachable while both legs of a trade come from one price and
  /// quantity; kept for the day a fee makes them differ.
  Conservation( ConserveError ),
  /// A post-only order would have taken liquidity. `book` is untouched.
  PostOnlyWouldTake,
}

impl core::fmt::Display for MatchError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::Quantity( error ) => write!( f, "quantity arithmetic failed during matching: {error}" ),
      Self::BookDesynchronized => write!( f, "the book refused a reduction or withdrawal the match loop had decided on" ),
      Self::Conservation( error ) => write!( f, "this batch of trades did not conserve: {error}" ),
      Self::PostOnlyWouldTake => write!( f, "a post-only order would have taken liquidity" ),
    }
  }
}

impl core::error::Error for MatchError
{
  fn source( &self ) -> Option< &( dyn core::error::Error + 'static ) >
  {
    match self
    {
      Self::Quantity( error ) => Some( error ),
      Self::Conservation( error ) => Some( error ),
      Self::BookDesynchronized | Self::PostOnlyWouldTake => None,
    }
  }
}

impl From< KindError > for MatchError
{
  fn from( error : KindError ) -> Self
  {
    Self::Quantity( error )
  }
}

impl From< ConserveError > for MatchError
{
  fn from( error : ConserveError ) -> Self
  {
    Self::Conservation( error )
  }
}

/// Whether `incoming` would trade on arrival — the best opposite price is
/// within its limit.
///
/// Price only: a self-match or an unsettleable fill could still stop the
/// trade, but for post-only the intent to take is what gets refused.
#[ must_use ]
pub fn would_take( book : &Book, incoming : &Order ) -> bool
{
  book.best( incoming.instrument, incoming.side.opposite() )
    .is_some_and( | best | side_accepts( incoming.side, incoming.price, best.order.price ) )
}

/// Whether the amount this fill pays can be expressed exactly.
///
/// Checked here, before the book is touched, for the same reason the submission
/// path reserves before it matches: `consume_best` is not undoable, so a fill
/// generated against a notional that cannot be expressed takes a resting order
/// off the book and gets nothing in return for it.
///
/// Fix(a_taking_buy_does_not_rest_across_an_ask_it_can_pay_for):
/// Root cause: a taking buy's `limit × quantity` was checked too, because
/// settlement computed it exactly. A fill paying an exact amount was declined,
/// and the buy rested above the ask.
/// Pitfall: settlement now rounds that product down; a check mirroring what
/// settlement computes has to change with it.
fn settleable( executed : Price, taken : Quantity ) -> bool
{
  notional( executed, taken ).is_ok()
}

/// Cross `incoming` against `book`, consuming what it matches.
///
/// The book keeps what was not traded: a consumed resting order leaves, a
/// partly consumed one keeps its place with less remaining. The incoming
/// order is never inserted; its remainder is the caller's.
///
/// `policy` resolves a pair sharing one account — see [`SelfMatchPolicy`] and
/// `exchange_core/docs/algorithm/002_self_match_prevention.md`.
/// `incoming.tif` matters only for [`exchange_tif::Tif::Fok`] and
/// [`exchange_tif::Tif::PostOnly`]; see the module doc's "Time-in-force".
///
/// # Errors
///
/// [`MatchError::PostOnlyWouldTake`] if `incoming` is post-only and
/// [`would_take`] holds, with `book` untouched. The other variants guard
/// states the loop cannot reach; see each.
pub fn cross( book : &mut Book, incoming : &Order, policy : SelfMatchPolicy ) -> Result< Crossing, MatchError >
{
  if !tif_takes( incoming.tif ) && would_take( book, incoming )
  {
    return Err( MatchError::PostOnlyWouldTake );
  }

  if tif_requires_full( incoming.tif )
  {
    // `cross_inner` is deterministic (see the module doc's "Determinism"
    // section), so probing a disposable clone first and, only on a full
    // fill, replaying the identical call against the real book is safe —
    // the replay cannot land anywhere the probe did not already go.
    let mut probe = book.clone();
    let would_fill = cross_inner( &mut probe, incoming, policy )?;

    if would_fill.remaining != Quantity::ZERO
    {
      // `book` was never touched — this is the same shape an ordinary
      // no-cross outcome already has, not a new case.
      return Ok( Crossing { trades : Vec::new(), remaining : incoming.quantity, cancelled : Vec::new() } );
    }
  }

  cross_inner( book, incoming, policy )
}

/// The crossing loop itself, shared by [`cross`]'s real call and its FOK
/// probe. See [`cross`] for everything this does not repeat.
fn cross_inner( book : &mut Book, incoming : &Order, policy : SelfMatchPolicy ) -> Result< Crossing, MatchError >
{
  let instrument = incoming.instrument;
  let opposite = incoming.side.opposite();
  let mut remaining = incoming.quantity;
  let mut trades = Vec::new();
  let mut cancelled = Vec::new();

  while remaining > Quantity::ZERO
  {
    // Best-first, always from the front. This loop never chooses a
    // counterparty; the book already did. Copied rather than borrowed so a
    // self-match resolution below is free to call back into `book`.
    let Some( best ) = book.best( instrument, opposite ).copied()
    else
    {
      break;
    };

    if !side_accepts( incoming.side, incoming.price, best.order.price )
    {
      break;
    }

    // A self-cross never trades, unconditionally — see the module
    // documentation's "Self-match prevention" section.
    if best.order.account == incoming.account
    {
      match policy
      {
        SelfMatchPolicy::CancelResting =>
        {
          withdraw( book, instrument, best.order.id )?;
          cancelled.push( SelfMatchCancellation { order : best.order.id, account : best.order.account, quantity : best.remaining, trades_before : trades.len() } );
          // The passed-over order was cancelled, not skipped, so nothing
          // ranks ahead of it that did not before — resume at whatever is
          // now the front of the book.
          continue;
        },
        SelfMatchPolicy::CancelIncoming =>
        {
          cancelled.push( SelfMatchCancellation { order : incoming.id, account : incoming.account, quantity : remaining, trades_before : trades.len() } );
          break;
        },
        SelfMatchPolicy::CancelBoth =>
        {
          withdraw( book, instrument, best.order.id )?;
          cancelled.push( SelfMatchCancellation { order : best.order.id, account : best.order.account, quantity : best.remaining, trades_before : trades.len() } );
          cancelled.push( SelfMatchCancellation { order : incoming.id, account : incoming.account, quantity : remaining, trades_before : trades.len() } );
          break;
        },
      }
    }

    let taken = remaining.min( best.remaining );
    let price = Trade::executed_price( best.order.price, incoming.price );

    // Declining an unsettleable fill is the third documented outcome — no
    // cross — reached for a second reason. Anything else would consume the
    // resting order below and then fail to pay for it.
    if !settleable( price, taken )
    {
      break;
    }

    let trade = Trade
    {
      taker : incoming.id,
      taker_account : incoming.account,
      taker_side : incoming.side,
      maker : best.order.id,
      maker_account : best.order.account,
      price,
      quantity : taken,
    };

    if !book.consume_best( instrument, opposite, taken )
    {
      return Err( MatchError::BookDesynchronized );
    }

    remaining = remaining.checked_sub( taken )?;
    trades.push( trade );
  }

  // Defense-in-depth, not a live check — see `MatchError::Conservation`'s
  // own doc comment for why this batch cannot actually fail it today.
  conserve_assert( &trades )?;

  Ok( Crossing { trades, remaining, cancelled } )
}

/// Withdraw the self-matched resting order `id`, which the loop just read at
/// the front of the book.
///
/// The book refusing means the two disagree about what rests. Ignored, the
/// loop would read the same order again: under cancel-resting, forever.
fn withdraw( book : &mut Book, instrument : InstrumentId, id : OrderId ) -> Result< (), MatchError >
{
  book.cancel( instrument, id ).map( drop ).ok_or( MatchError::BookDesynchronized )
}
