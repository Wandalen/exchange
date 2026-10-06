//! The crossing engine: one incoming order against a book, producing trades.
//!
//! Three outcomes, and all three are outcomes rather than errors:
//!
//! - **Full fill** — the incoming quantity is entirely consumed; nothing rests.
//! - **Partial fill** — some trades, and a remainder to dispose of.
//! - **No cross** — zero trades, the whole quantity remaining. The book had
//!   nothing at an acceptable price, or nothing it could be paid for exactly.
//!
//! The third one is the one worth naming. An engine that always finds a match
//! passes every test about matching and is catastrophically wrong; *declining*
//! is behaviour, and it is the behaviour the smoke lane's control arm exists
//! to observe.
//!
//! # Why a fill can be declined on price it *does* cross
//!
//! A quantity times a price is a product at twice the scale, and
//! exact_arith refuses to round it — [`exchange_types::notional`] returns
//! `NotionalInexact` rather than pick a direction, because rounding a fill is
//! how an exchange creates or destroys the fraction it rounded. That refusal is
//! reachable on a *partial* fill even when both orders were accepted: a sell
//! reserves the asset and is never asked for a notional at all, so a resting
//! ask can carry a quantity whose value at its own price needs a seventh
//! decimal. Settlement would then fail — and it fails *after* the book has
//! already given up the resting order, which no later step can undo.
//!
//! So the check runs here, before [`exchange_book::Book::consume_best`], and an
//! unsettleable fill is declined instead of generated, by this crate's own
//! `settleable` predicate.
//!
//! **The consequence, named rather than hidden:** two orders can now rest at
//! prices that cross, because the only fill between them is one neither side
//! can be paid for. A crossed book is an anomaly, and it is the deliberate
//! trade — it is visible, and either side can cancel and recover its
//! reservation. What it replaces was not recoverable at all. Closing it
//! properly needs a minimum fill increment derived from the price, which is a
//! decision this crate has not taken.
//!
//! # Self-match prevention
//!
//! A candidate pair sharing one account never trades — fixed, never a mode,
//! per the family's self-match-prevention design. The check compares the
//! incoming order's account against each candidate resting order's, in that
//! order, before any [`Trade`] for the pair is built — never after, and
//! ahead of the settleability check below it, since a pair this forbids from
//! trading has no need to also be asked whether it could have settled. What
//! happens to the passed-over side is the one configurable part —
//! [`SelfMatchPolicy`] names the three choices — and this crate holds no
//! state between calls, so the caller supplies it on every call rather than
//! this crate storing one per book.
//!
//! # What this crate does not decide
//!
//! It does not decide who is next — [`exchange_book`] hands out its side
//! best-first and this crate takes from the front. It does not decide the
//! executed price — [`exchange_fill::Trade::executed_price`] owns that rule.
//! It does not touch escrow: reservations move in the escrow ledger, driven
//! by the trades this crate returns. Splitting it this way is what keeps the
//! matching loop
//! small enough to read in one sitting.
//!
//! # Determinism
//!
//! Nothing here reads a clock, iterates a hash container, or compares an
//! address. The loop's only inputs are the incoming order and the book's
//! published order, so the same accepted sequence against the same starting
//! book produces the same trades — which is what makes a fill re-derivable
//! from the record, and therefore auditable at all.
//!
//! # Extraction
//!
//! [`SelfMatchPolicy`] moved out to its own root crate, `exchange_stp`, and
//! is re-exported here unchanged — see that crate's module documentation for
//! why it keeps the real three variants rather than the source design's
//! `Allow`/`CancelOldest`/`CancelNewest`. [`SelfMatchCancellation`] stays
//! here: it carries this crate's own `AccountId`/`OrderId`/`Quantity`, not
//! pure self-match vocabulary, so it belongs with the match outcome rather
//! than the policy that caused it.
//!
//! # Time-in-force
//!
//! `incoming.tif` is read in exactly one place: [`tif_requires_full`] gates
//! whether a partial outcome is acceptable at all. IOC needs no logic here —
//! this function has never inserted the incoming order's remainder itself
//! (see "What this crate does not decide" above), so "drop any remainder"
//! was already true for every TIF; only whether a *caller* later rests that
//! remainder differs, which is `exchange_rest`/`exchange_core`'s decision,
//! not this crate's.
//!
//! FOK is different: by the time this function returns, any trade it made is
//! real — a caller cannot un-happen a partial fill. So a FOK order is tried
//! first against a disposable clone of `book`; only if that probe fully
//! consumes it does the identical, deterministic sequence run again against
//! the real `book`. An unfillable FOK comes back shaped exactly like an
//! ordinary no-cross outcome — empty trades, full remaining, nothing
//! cancelled — because that is what actually happened to `book`: nothing.
//! Translating "FOK, nothing filled" into a rejection is
//! `exchange_core`'s job, same as every other outcome this crate reports
//! rather than judges.

use exact_arith::{ KindError, Quantity };
use exchange_book::Book;
use exchange_conserve::{ conserve_assert, ConserveError };
use exchange_fill::Trade;
use exchange_id::{ AccountId, OrderId };
use exchange_order::Order;
use exchange_side::{ Side, side_accepts };
use exchange_tif::tif_requires_full;
use exchange_types::{ Price, notional };

pub use exchange_stp::SelfMatchPolicy;

/// What crossing an order against a book produced.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct Crossing
{
  /// The trades generated, in the order they were generated.
  pub trades : Vec< Trade >,
  /// What is left of the incoming order. Zero on a full fill.
  ///
  /// Still counts a self-match-cancelled remainder — `filled() + remaining`
  /// sums to the incoming order's own submitted quantity regardless of why
  /// anything was left over. If `cancelled` names the incoming order's own
  /// id, this quantity was withdrawn, not left for the caller to rest — see
  /// [`SelfMatchCancellation`].
  pub remaining : Quantity,
  /// Every order withdrawn by self-match prevention during this call, in the
  /// order it happened. Empty whenever no candidate pair ever shared a
  /// self-match key.
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
  /// [`KindError`] if the trades' quantities do not sum within the quantity
  /// type — unreachable for trades this crate produced, since they are all
  /// splits of one submitted quantity, but the sum is checked rather than
  /// assumed because this is also usable on a caller's own collection.
  pub fn filled( &self ) -> Result< Quantity, KindError >
  {
    self.trades.iter().try_fold( Quantity::ZERO, | sum, trade | sum.checked_add( trade.quantity ) )
  }
}

/// One order [`SelfMatchPolicy`] withdrew rather than let trade or rest.
///
/// Always cause self-match — see `exchange_fill::CancelCause::SelfMatch`.
/// This crate does not construct that event itself, since it never touches
/// escrow or the event stream (see the module documentation); it reports
/// what happened so a caller that does can.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct SelfMatchCancellation
{
  /// The order withdrawn — either side of the self-matching pair.
  pub order : OrderId,
  /// The account both sides of the pair shared.
  pub account : AccountId,
  /// How much of it was withdrawn.
  pub quantity : Quantity,
}

/// Something the matching loop could not do.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum MatchError
{
  /// A quantity arithmetic step failed. Every quantity here is a split of the
  /// incoming order's own quantity, so this is unreachable in practice — it
  /// is returned rather than unwrapped because an engine that panics mid-match
  /// leaves the book in a state no event describes.
  Quantity( KindError ),
  /// The book refused a reduction the loop had already decided on. Means the
  /// book and this loop disagree about what rests, which no later step could
  /// repair.
  BookDesynchronized,
  /// `exchange_conserve::conserve_assert` refused this call's own batch of
  /// trades. Both of a trade's legs are built from the same `price`/
  /// `quantity` pair (see [`Trade::executed_price`]), so they offset by
  /// construction — unreachable through this function today, kept as
  /// defense-in-depth for the day a fee or similar asymmetry enters the
  /// family's design. See `exchange_conserve`'s own module doc, "Revision"
  /// section.
  Conservation( ConserveError ),
}

impl core::fmt::Display for MatchError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::Quantity( error ) => write!( f, "quantity arithmetic failed during matching: {error}" ),
      Self::BookDesynchronized => write!( f, "the book refused a reduction the match loop had decided on" ),
      Self::Conservation( error ) => write!( f, "this batch of trades did not conserve: {error}" ),
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
      Self::BookDesynchronized => None,
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

/// Whether every currency amount this fill implies can be expressed exactly.
///
/// Two amounts, not one. The trade settles at the maker's price, and a *taking
/// buy* is additionally handed back the difference between its own limit and
/// that price — a second notional, at a second price. Settlement computes both
/// before it moves anything, so either being inexpressible makes the whole fill
/// unsettleable.
///
/// Checked here, before the book is touched, for the same reason the submission
/// path reserves before it matches: `consume_best` is not undoable, so a fill
/// generated against a notional that cannot be expressed takes a resting order
/// off the book and gets nothing in return for it.
///
/// A taking *sell* needs only the one check: the maker is the buyer then, and a
/// maker executes at its own price, so the two notionals coincide.
fn settleable( incoming : &Order, executed : Price, taken : Quantity ) -> bool
{
  if notional( executed, taken ).is_err()
  {
    return false;
  }

  match incoming.side
  {
    Side::Buy => notional( incoming.price, taken ).is_ok(),
    Side::Sell => true,
  }
}

/// Cross `incoming` against `book`, consuming what it matches.
///
/// The book is left holding exactly what was not traded: fully consumed
/// resting orders are removed, a partially consumed one keeps its position
/// with a reduced remainder. The incoming order is **not** inserted — what to
/// do with a remainder is the caller's decision, not this crate's.
///
/// `policy` resolves any candidate pair that shares a self-match key, per
/// `docs/algorithm/002_self_match_prevention.md` — see [`SelfMatchPolicy`].
///
/// `incoming.tif` matters only when it is [`exchange_tif::Tif::Fok`] — see the
/// module doc's "Time-in-force" section for why every other value needs no
/// special handling here.
///
/// # Errors
///
/// [`MatchError`] if a quantity step fails, the book disagrees with the loop
/// about what rests, or this call's own batch of trades fails
/// [`exchange_conserve::conserve_assert`]. None of the three is reachable
/// through the exchange engine's submission path — the third is checked as
/// defense-in-depth regardless; see [`MatchError::Conservation`]'s own doc.
pub fn cross( book : &mut Book, incoming : &Order, policy : SelfMatchPolicy ) -> Result< Crossing, MatchError >
{
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
          let _ = book.cancel( instrument, best.order.id );
          cancelled.push( SelfMatchCancellation { order : best.order.id, account : best.order.account, quantity : best.remaining } );
          // The passed-over order was cancelled, not skipped, so nothing
          // ranks ahead of it that did not before — resume at whatever is
          // now the front of the book.
          continue;
        },
        SelfMatchPolicy::CancelIncoming =>
        {
          cancelled.push( SelfMatchCancellation { order : incoming.id, account : incoming.account, quantity : remaining } );
          break;
        },
        SelfMatchPolicy::CancelBoth =>
        {
          let _ = book.cancel( instrument, best.order.id );
          cancelled.push( SelfMatchCancellation { order : best.order.id, account : best.order.account, quantity : best.remaining } );
          cancelled.push( SelfMatchCancellation { order : incoming.id, account : incoming.account, quantity : remaining } );
          break;
        },
      }
    }

    let taken = remaining.min( best.remaining );
    let price = Trade::executed_price( best.order.price, incoming.price );

    // Declining an unsettleable fill is the third documented outcome — no
    // cross — reached for a second reason. Anything else would consume the
    // resting order below and then fail to pay for it.
    if !settleable( incoming, price, taken )
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
