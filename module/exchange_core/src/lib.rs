//! The submission path — the crate a caller uses, and the only one that knows
//! all five parts of the family exist.
//!
//! The Contract is one line: *orders in → trades + event stream out; escrow
//! holds; no ECS types anywhere*. [`Exchange`] is where those three clauses
//! meet. Everything else here is re-export.
//!
//! # The order of operations is the design
//!
//! [`Exchange::submit`] does five things and their order is load-bearing:
//!
//! 1. **Validate.** A malformed order is rejected whole.
//! 2. **Reserve**, before anything is visible to matching. An order that
//!    cannot be fully reserved never reaches step 3, so the book never
//!    displays an order that could fail to settle.
//! 3. **Match**, best-first against the opposite side.
//! 4. **Settle each trade**, moving reserved units to their counterparties in
//!    the same step that generated them — never in a later pass that could
//!    lag or be lost.
//! 5. **Rest the remainder**, reservation retained.
//!
//! Reversing 2 and 3 is the tempting simplification and it is the expensive
//! one: matching first and reserving after means a fill can be generated
//! against funds that were never there, and the failure surfaces as an
//! unattributable imbalance long after the trade that caused it.
//!
//! # Every step emits its event
//!
//! No state changes without one. The event stream is the Contract's second
//! output and the audit record both, so a mutation with no event is a fact the
//! record cannot reproduce.
//!
//! # What this slice does not implement
//!
//! Named rather than silently absent, because the family's own design
//! documents specify all of them and a reader deserves to know which parts are
//! real:
//!
//! - **Time-in-Force.** `{ FOK, IOC, GTC }` is specified and [`Order`] now
//!   carries a real `tif` field — but [`Exchange::submit`] pins every order
//!   it constructs to `Tif::Gtc` rather than taking a caller-supplied value,
//!   because `cross` does not consult the field yet. FOK needs a pre-check
//!   against visible liquidity and IOC needs a cancel-the-remainder
//!   disposition — both are real work the matching crate owes this facade,
//!   not something a hardcoded value here can stand in for.
//! - **Multi-instrument.** [`Book`] itself is keyed by instrument now, but
//!   this facade still pins every order it constructs to one constant
//!   [`exchange_id::InstrumentId`] rather than taking a caller-supplied
//!   value — nothing here routes a submission to any instrument but that
//!   one, so from a caller's perspective the facade is still single-market.
//!   Exposing the choice is a facade-level change this crate has not made.
//! - **Market orders.** Only limit orders exist. A market order is a limit
//!   order with no price bound, which changes the crossing predicate and the
//!   reservation rule together; nothing here exercises one.
//! - **Amend and fees.** Both are specified at family grain. Fees in
//!   particular have a decided hook position and an undecided schedule, so
//!   implementing the hook now would be building a parameter nobody can
//!   supply.
//! - **Self-match policy is fixed, not yet per-book configurable.** Detection
//!   and cancellation are real — `docs/algorithm/002_self_match_prevention.md`
//!   is implemented — but the three-policy choice that document specifies as
//!   configurable per book is hardcoded below to `SelfMatchPolicy::CancelIncoming`,
//!   its own named conservative candidate, because this crate has no
//!   per-book configuration surface to select one from yet.
//! - **Concurrent intake.** The arrival sequence here is a counter incremented
//!   by one submitting thread. The family's design puts the real total
//!   order in a shared merge substrate; what this crate owes that design is
//!   that no decision on the matching path reads anything the sequence does
//!   not carry, and that obligation *is* met — nothing here reads a clock,
//!   iterates a hash container, or compares an address.

pub use exact_arith::
{
  Backing, ConservationError, Entry, KindError, MONEY_SCALE, Money, Quantity, Report, verify,
};
pub use exchange_book::{ Book, Resting };
pub use exchange_escrow::{ Account, Conserved, Escrow, EscrowError, Holding };
pub use exchange_id::InstrumentId;
pub use exchange_match::{ Crossing, MatchError, SelfMatchCancellation, SelfMatchPolicy };
use exchange_seq::seq_next;
use exchange_tif::Tif;
pub use exchange_types::
{
  AccountId, Amount, CancelCause, Event, EventKind, Obligation, Order, OrderId, Price, RejectReason,
  Sequence, Side, Trade, TypeError, notional, obligation,
};

/// The one instrument every order books against, until the book itself is
/// keyed by instrument. See the module doc's "Multi-instrument" bullet.
const SINGLE_INSTRUMENT : InstrumentId = InstrumentId( 1 );

/// What came of a submission.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct Receipt
{
  /// The id assigned to the submitted order.
  pub order : OrderId,
  /// The trades it generated, in the order they were generated. Empty when
  /// the order crossed nothing — which is an outcome, not a failure.
  pub trades : Vec< Trade >,
  /// What is resting on the book afterwards. Zero on a full fill, and also
  /// zero when self-match prevention withdrew the order's own remainder
  /// instead of resting it — see [`Self::self_match_cancelled`].
  pub resting : Quantity,
  /// Whether `resting` is zero because self-match prevention cancelled the
  /// order's own remainder, rather than because every unit found a
  /// counterparty. [`Self::is_complete`] uses this to tell the two apart.
  pub self_match_cancelled : bool,
}

impl Receipt
{
  /// Whether the order was entirely filled and nothing rests.
  ///
  /// Fix(is_complete_reported_a_cancellation_as_a_fill):
  /// Root cause: `resting` reaches zero two different ways — every unit
  /// found a counterparty, or `Exchange::submit` step 4a withdrew whatever
  /// was left instead of resting it — and this method returned `true` for
  /// both, contradicting its own "entirely filled" doc for the second: a
  /// self-match-cancelled order that traded nothing reported as complete.
  ///
  /// Pitfall: `resting == Quantity::ZERO` answers "is anything left to
  /// rest", not "did this fill" — the two coincide only when nothing was
  /// cancelled, so any later outcome that zeroes `resting` without a fill
  /// needs its own flag here too, not a widened zero-check.
  #[ must_use ]
  pub fn is_complete( &self ) -> bool
  {
    self.resting == Quantity::ZERO && !self.self_match_cancelled
  }
}

/// Something the exchange refused.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum ExchangeError
{
  /// The order was refused before it could rest. The reason is recorded in
  /// the event stream as well as returned here.
  Rejected( RejectReason ),
  /// Escrow refused a move.
  Escrow( EscrowError ),
  /// Matching failed.
  Matching( MatchError ),
  /// A cancel or a lookup named an order that is not resting — a race result
  /// rather than a caller error, reported rather than ignored.
  NotResting( OrderId ),
}

impl core::fmt::Display for ExchangeError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::Rejected( reason ) => write!( f, "the order was rejected: {reason:?}" ),
      Self::Escrow( error ) => write!( f, "escrow refused: {error}" ),
      Self::Matching( error ) => write!( f, "matching failed: {error}" ),
      Self::NotResting( id ) => write!( f, "order {} is not resting", id.0 ),
    }
  }
}

impl core::error::Error for ExchangeError
{
  fn source( &self ) -> Option< &( dyn core::error::Error + 'static ) >
  {
    match self
    {
      Self::Escrow( error ) => Some( error ),
      Self::Matching( error ) => Some( error ),
      Self::Rejected( _ ) | Self::NotResting( _ ) => None,
    }
  }
}

impl From< EscrowError > for ExchangeError
{
  fn from( error : EscrowError ) -> Self
  {
    Self::Escrow( error )
  }
}

impl From< MatchError > for ExchangeError
{
  fn from( error : MatchError ) -> Self
  {
    Self::Matching( error )
  }
}

/// One market: a book, the balances behind it, and the record of everything
/// that happened.
#[ derive( Debug, Clone, Default ) ]
pub struct Exchange
{
  book : Book,
  escrow : Escrow,
  next_order : u64,
  next_sequence : Sequence,
  events : Vec< Event >,
}

impl Exchange
{
  /// A market with no participants and an empty book.
  #[ must_use ]
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Give `id` a starting balance. If `id` is already open, `cash` and
  /// `asset` are added to what remains available — see [`Escrow::open`].
  ///
  /// # Errors
  ///
  /// [`ExchangeError::Escrow`] if the deposit does not fit the account's
  /// existing balance.
  pub fn open_account( &mut self, id : AccountId, cash : Money, asset : Quantity ) -> Result< (), ExchangeError >
  {
    self.escrow.open( id, cash, asset )?;
    Ok( () )
  }

  /// Submit a limit order, matching it against the book and resting whatever
  /// is left.
  ///
  /// # Errors
  ///
  /// [`ExchangeError::Rejected`] when validation, reservation, settlement, or
  /// a self-match release refuses the order — in which case an
  /// [`EventKind::OrderRejected`] is recorded and no other state (book,
  /// escrow, or any other event) ever changed. Every one of those steps runs
  /// first as a dry run against scratch state before any of it commits for
  /// real, so a failure discovered mid-crossing still reports as one clean,
  /// atomic rejection. [`ExchangeError::Matching`] for a failure the design
  /// does not consider reachable through this path.
  pub fn submit
  (
    &mut self,
    account : AccountId,
    side : Side,
    price : Price,
    quantity : Quantity,
  ) -> Result< Receipt, ExchangeError >
  {
    let order = Order
    {
      id : self.claim_order(), instrument : SINGLE_INSTRUMENT, account, side, price, quantity,
      tif : Tif::Gtc,
    };

    // 1. Validate.
    if quantity == Quantity::ZERO
    {
      return Err( self.reject( &order, RejectReason::ZeroQuantity ) );
    }
    // A negative price has to be refused here rather than left to escrow,
    // because escrow only ever sees it on one of the two sides. A negative
    // buy has a negative notional and step 2 refuses it; a negative *sell*
    // reserves the asset, whose amount the price never enters, so it rests
    // perfectly well — and then destroys the first order that crosses it, in
    // step 4, after `cross` has already mutated the book and after the
    // aggressor's own reservation has been taken. There is no unwind there:
    // the `?` returns, the reservation is stranded with no order to cancel
    // against, and the book keeps whatever the match did to it, all without
    // emitting the event this crate's protocol says accompanies every state
    // change.
    //
    // Fix(a_negative_price_is_refused_before_it_can_rest):
    // Root cause: `Price` is `Money`, which is signed, and the only guard
    // between a price and settlement lived in escrow — which a sell's
    // obligation bypasses entirely, since it is denominated in asset.
    // Pitfall: validating a field where its *value* is consumed misses every
    // path that carries the field past that point without consuming it. A
    // resting sell carries its price across the whole matching step untouched.
    if price < Money::ZERO
    {
      return Err( self.reject( &order, RejectReason::NegativePrice ) );
    }

    // 2. Dry-run the whole operation — reserve, cross, settle every trade,
    // release every self-match cancellation — against scratch clones, before
    // any of it touches real state.
    //
    // Fix(exchange_submit_partial_crossing_could_strand_state):
    // `cross` mutates the book it is given as it computes a crossing,
    // unconditionally withdrawing every resting order it matches, and
    // `Holding::receive` is an unconditional, non-reservation-bounded
    // credit — so a second trade in one crossing can breach an account's
    // ceiling even though the first trade's `settle` and its emitted event
    // already committed for real. Without this dry run, that later failure
    // left an `OrderAccepted` event on record for an order the caller never
    // received an id for, real proceeds already paid out for whichever
    // trades did settle, and every resting order `cross` consumed —
    // including the one behind the failed trade — gone from the book with
    // no trade, no event, and no way back. `cross` is a pure function of the
    // book and the incoming order (`exchange_match`'s own
    // `crossing_the_same_book_twice_gives_the_same_trades`), so replaying it
    // against the real book below reproduces this dry run exactly.
    // Root cause: real state was mutated one step at a time, each failure
    // guarded individually with `?`, instead of the whole operation being
    // validated as one atomic unit before any of it committed.
    // Pitfall: dry-running only the crossing/settlement portion and leaving
    // step 2's reserve and its event committing unconditionally still
    // strands the incoming order's own reservation on a mid-crossing
    // failure — the dry run has to cover reserve through release, not just
    // cross and settle.
    let mut dry_escrow = self.escrow.clone();
    if let Err( error ) = dry_escrow.reserve( &order )
    {
      return Err( self.reject( &order, Self::reason_for( error ) ) );
    }
    let mut dry_book = self.book.clone();
    let dry_crossing = exchange_match::cross( &mut dry_book, &order, SelfMatchPolicy::CancelIncoming )?;
    for trade in &dry_crossing.trades
    {
      if let Err( error ) = dry_escrow.settle( trade, side, price )
      {
        return Err( self.reject( &order, Self::reason_for( error ) ) );
      }
    }
    for cancellation in &dry_crossing.cancelled
    {
      if let Err( error ) = dry_escrow.release( cancellation.account, cancellation.order )
      {
        return Err( self.reject( &order, Self::reason_for( error ) ) );
      }
    }

    // 3. The dry run above succeeded in full — replay it for real. Every
    // call below is guaranteed to succeed: same starting state, the same
    // pure `cross`, and nothing else touches `self.escrow`/`self.book` in
    // between.
    let reserved = self.escrow.reserve( &order ).expect( "already validated by the dry run above" );
    let arrival = self.emit( order.id, order.account, EventKind::OrderAccepted { side, price, quantity, reserved } );
    let crossing = exchange_match::cross( &mut self.book, &order, SelfMatchPolicy::CancelIncoming )
      .expect( "already produced by the identical dry run above — cross is pure" );

    // 4. Settle each trade in the step that generated it. Already validated
    // by the identical dry-run settle loop above, against the same starting
    // escrow state and the same trades `cross` just reproduced.
    for trade in &crossing.trades
    {
      self.escrow.settle( trade, side, price ).expect( "already validated by the dry run above" );
      self.emit( order.id, order.account, EventKind::Trade( *trade ) );
    }

    // 4a. Release and report each self-match cancellation. The resting side
    // of any such cancellation is already gone from the book by this point —
    // `exchange_match::cross` withdrew it before returning. If the incoming
    // order itself was the cancelled side, note it: its remainder was just
    // released here rather than being available to rest in step 5.
    let mut incoming_cancelled = false;
    for cancellation in &crossing.cancelled
    {
      let released = self.escrow.release( cancellation.account, cancellation.order )
        .expect( "already validated by the dry run above" );
      self.emit
      (
        cancellation.order, cancellation.account,
        EventKind::OrderCancelled { cause : CancelCause::SelfMatch, quantity : cancellation.quantity, released },
      );
      incoming_cancelled |= cancellation.order == order.id;
    }

    // 5. Rest the remainder, reservation retained. `crossing.remaining` alone
    // cannot distinguish "nothing left to cross" from "the incoming order's
    // remainder was just cancelled above" — both leave it positive — so the
    // flag from 4a decides whether anything is left to rest at all.
    let resting = if incoming_cancelled { Quantity::ZERO } else { crossing.remaining };
    if resting > Quantity::ZERO
    {
      assert!
      (
        self.book.insert( Resting { order, remaining : resting, arrival } ),
        "order id came from this exchange's own next_order counter, which never repeats",
      );
    }

    Ok( Receipt { order : order.id, trades : crossing.trades, resting, self_match_cancelled : incoming_cancelled } )
  }

  /// Withdraw a resting order's remainder and return its reservation.
  ///
  /// # Errors
  ///
  /// [`ExchangeError::NotResting`] if no such order rests — which happens
  /// when it filled or was cancelled first, and is a defined outcome rather
  /// than a caller error. On [`ExchangeError::Escrow`], the order is left
  /// resting exactly as it was — cancellation only commits once the release
  /// it depends on has actually succeeded.
  ///
  /// Fix(a_cancel_removed_the_order_before_the_release_could_fail):
  /// Root cause: `book.cancel` is infallible once an order is found — it
  /// removes and returns it unconditionally — while the `escrow.release`
  /// that followed it could still fail, most reachably with
  /// `EscrowError::Arithmetic` when the account's `available` balance sits
  /// close enough to the declared ceiling that crediting the reservation
  /// back would breach it. On that failure the function returned `Err`, but
  /// the order was already gone from the book: not filled, not cancelled,
  /// not resting, unreachable by any public API and permanently stuck with
  /// its reservation. The same commit-before-validate shape already fixed
  /// once in `Escrow::release` itself, one level up the call stack.
  ///
  /// Pitfall: an infallible-looking call ahead of a fallible one reads as
  /// safe ordering — "first find it, then let it go" — right up until the
  /// second call's failure mode is traced to something the first call
  /// cannot undo.
  pub fn cancel( &mut self, id : OrderId ) -> Result< Quantity, ExchangeError >
  {
    let ( account, instrument ) = self.book.iter().find( | resting | resting.order.id == id )
      .map( | resting | ( resting.order.account, resting.order.instrument ) )
      .ok_or( ExchangeError::NotResting( id ) )?;
    let released = self.escrow.release( account, id )?;
    let resting = self.book.cancel( instrument, id )
      .expect( "just confirmed resting above, and release does not touch the book" );

    self.emit
    (
      resting.order.id, resting.order.account,
      EventKind::OrderCancelled { cause : CancelCause::Request, quantity : resting.remaining, released },
    );

    Ok( resting.remaining )
  }

  /// Everything that has happened, in order.
  #[ must_use ]
  pub fn events( &self ) -> &[ Event ]
  {
    &self.events
  }

  /// The resting book.
  #[ must_use ]
  pub fn book( &self ) -> &Book
  {
    &self.book
  }

  /// The balances behind it.
  #[ must_use ]
  pub fn escrow( &self ) -> &Escrow
  {
    &self.escrow
  }

  /// Every trade as a pair of currency postings — the buyer debited, the
  /// seller credited — ready for [`verify`].
  ///
  /// This is the bridge to `exact_arith`'s conservation auditor, and it is
  /// worth having for one reason: the auditor was built to grade a transaction
  /// log it knows nothing about, so running it here checks the exchange with
  /// machinery the exchange did not write. A conservation test the exchange
  /// authored itself would agree with the exchange by construction.
  ///
  /// # Errors
  ///
  /// [`ExchangeError::Rejected`] carrying
  /// [`RejectReason::ObligationUnrepresentable`] if a trade's own notional
  /// cannot be expressed, which would mean it should never have executed.
  pub fn postings( &self ) -> Result< Vec< Entry >, ExchangeError >
  {
    let mut entries = Vec::new();

    for event in &self.events
    {
      let EventKind::Trade( trade ) = event.kind
      else
      {
        continue;
      };

      let paid = notional( trade.price, trade.quantity )
      .map_err( | _ | ExchangeError::Rejected( RejectReason::ObligationUnrepresentable ) )?;

      let ( buyer, seller ) = self.parties( &trade );
      entries.push( Entry::new( format!( "account:{}", buyer.0 ), -paid.minor() ) );
      entries.push( Entry::new( format!( "account:{}", seller.0 ), paid.minor() ) );
    }

    Ok( entries )
  }

  /// Which of a trade's two accounts bought, and which sold.
  ///
  /// Read back from the aggressor's own `OrderAccepted` event rather than from
  /// the book, because by the time this is called both orders may be long
  /// gone — a filled order leaves no book state at all, and postings must
  /// still be derivable from the record alone.
  fn parties( &self, trade : &Trade ) -> ( AccountId, AccountId )
  {
    let taker_bought = self.events.iter().any( | event |
    {
      event.order == trade.taker
      && matches!( event.kind, EventKind::OrderAccepted { side : Side::Buy, .. } )
    } );

    if taker_bought
    {
      ( trade.taker_account, trade.maker_account )
    }
    else
    {
      ( trade.maker_account, trade.taker_account )
    }
  }

  fn claim_order( &mut self ) -> OrderId
  {
    let id = OrderId( self.next_order );
    self.next_order += 1;
    id
  }

  /// Record an event and hand back the position it claimed.
  ///
  /// The position is claimed here and nowhere else, so it is a counter and
  /// never a clock reading. Takes `id`/`account` rather than `&Order` because
  /// a self-match cancellation of the *resting* side has only those two —
  /// `exchange_match::cross` has already withdrawn it from the book by the
  /// time its cancellation is reported, so no full `Order` is available to
  /// reconstruct.
  fn emit( &mut self, order : OrderId, account : AccountId, kind : EventKind ) -> Sequence
  {
    let sequence = self.next_sequence;
    self.next_sequence = seq_next( sequence );
    self.events.push( Event { sequence, order, account, kind } );
    sequence
  }

  fn reject( &mut self, order : &Order, reason : RejectReason ) -> ExchangeError
  {
    self.emit( order.id, order.account, EventKind::OrderRejected { reason } );
    ExchangeError::Rejected( reason )
  }

  /// Map what `Escrow::reserve` refused, in step 2 of [`Self::submit`], to
  /// the reason reported to the caller.
  ///
  /// An exhaustive match, not a wildcard: a reason no code path produces is a
  /// claim the code does not back, and letting a future `EscrowError` variant
  /// fall through unnoticed into a generic bucket would quietly repeat the
  /// mistake this function was once found making.
  ///
  /// Fix(a_reachable_ceiling_breach_was_reported_as_a_funds_shortfall):
  /// Root cause: `Holding::reserve` checks `available` and `reserved`
  /// independently — `available.checked_minus(amount)` for a genuine
  /// shortfall, then `reserved.checked_plus(amount)` for whether the
  /// account's *pooled* commitment still fits the representable ceiling.
  /// This function's catch-all mapped every `EscrowError` other than
  /// `UnknownAccount`/`Obligation` to `RejectReason::InsufficientFunds`, on
  /// the doc-commented claim that nothing else was reachable from `submit`.
  /// `EscrowError::Arithmetic` from the second check is reachable: an account
  /// that already holds one large reservation and then earns fresh cash
  /// settling as a seller can pass the first check on real, spendable funds
  /// and still fail the second.
  ///
  /// Pitfall: a claim of unreachability stated once, for a whole match arm at
  /// a time, survives exactly until one of the variants it bundles together
  /// turns out to have its own path in. The fix here is not "add a check" —
  /// it is separating one variant proven reachable from the others that
  /// remain genuinely dead, so the next such proof only has to re-examine one
  /// arm instead of re-deriving the whole bundle.
  fn reason_for( error : EscrowError ) -> RejectReason
  {
    match error
    {
      EscrowError::UnknownAccount( _ ) => RejectReason::UnknownAccount,
      EscrowError::Obligation( _ ) => RejectReason::ObligationUnrepresentable,
      // The account's funds were genuinely sufficient; a fixed-point ceiling
      // was not — whether at reservation time, or when a trade's own
      // proceeds would themselves breach one during the dry-run `settle`
      // this function's own doc comment describes.
      EscrowError::Arithmetic => RejectReason::ReservationUnrepresentable,
      // `AlreadyReserved` and `Obligation` stay reserve-only regardless of
      // this function's now-wider callers: the order id is freshly claimed
      // so nothing could already hold a reservation under it, and neither
      // `settle` nor `release` ever computes a fresh obligation — both work
      // from what `reserve` already stored. `NegativeAmount` is foreclosed
      // for `reserve` by step 1 (a buy's notional is negative exactly when
      // its price is, refused before escrow is reached) and for
      // `settle`/`release` because every amount they move is already
      // validated non-negative before they see it. `NotReserved`,
      // `NoReservation`, and `ObligationMismatch` are reachable in principle
      // from `settle`'s own `reduced_obligation` and from `release`, but not
      // through `submit`'s dry run specifically: `settle` writes each
      // trade's reduced remainder back to the per-order ledger before the
      // next trade or cancellation for that same order is ever considered,
      // so a later self-match release always finds a still-live,
      // correctly-shaped reservation for whatever `settle` left behind.
      // Reported as insufficient funds rather than given a reason variant of
      // its own, because a reason no code path produces through this
      // function is a claim the code does not back.
      EscrowError::Insufficient
      | EscrowError::NotReserved
      | EscrowError::NoReservation( _ )
      | EscrowError::AlreadyReserved( _ )
      | EscrowError::ObligationMismatch
      | EscrowError::NegativeAmount => RejectReason::InsufficientFunds,
    }
  }
}
