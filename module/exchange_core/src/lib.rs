//! The submission path — the crate a caller uses, and the only one that knows
//! every part of the family exists.
//!
//! The Contract is one line: *orders in → trades + event stream out; escrow
//! holds; no ECS types anywhere*. [`Exchange`] is where those three clauses
//! meet. Everything else here is re-export.
//!
//! # The order of operations is the design
//!
//! [`Exchange::exchange_step`]'s own per-command pipeline does five things and
//! their order is load-bearing:
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
//! # `exchange_step` replaces `submit`
//!
//! The old `Exchange::submit( account, side, price, quantity )` is gone.
//! [`Exchange::exchange_step`] drains an [`exchange_inbound`] ring and applies
//! every [`InboundCmd`] it finds, in drain order — see that method's own doc
//! for the full design (why it owns sequencing rather than trusting a
//! caller-supplied id/arrival, why `Cancel` delegates to [`Exchange::cancel`]
//! unchanged, and why `Replace` is not yet wired). Three of the four bullets
//! the previous revision of this doc named as "not implemented" are resolved
//! as a direct consequence, not a separate effort:
//!
//! - **Time-in-Force** is a real, caller-chosen field on every [`InboundCmd::Place`]'s
//!   own [`Order`] now — `exchange_match::cross` has consulted `tif` since the
//!   crate's own TIF/FOK rework, so there is nothing left pinning it to `Gtc`.
//! - **Multi-instrument** is real: `exchange_step` reads `instrument` off the
//!   incoming order itself rather than a facade-wide constant. [`Book`] was
//!   already keyed by instrument; this was the last piece pinning every
//!   submission to one of them regardless.
//! - **Self-match policy** is a parameter on [`Exchange::exchange_step`]
//!   itself, supplied per call exactly like `exchange_match::cross`'s own
//!   `policy` parameter — never stored on `Exchange`, matching the rest of
//!   this family's "no state between calls, the caller supplies it" policy
//!   convention.
//!
//! # What this slice still does not implement
//!
//! Named rather than silently absent:
//!
//! - **Market orders.** Only limit orders exist. A market order is a limit
//!   order with no price bound, which changes the crossing predicate and the
//!   reservation rule together; nothing here exercises one.
//! - **Amend and fees.** Both are specified at family grain. Fees in
//!   particular have a decided hook position and an undecided schedule, so
//!   implementing the hook now would be building a parameter nobody can
//!   supply.
//! - **Replace, end to end.** [`exchange_inbound::InboundCmd::Replace`] exists
//!   and is tested at the book level in that crate's own suite, but
//!   `exchange_step` does not yet apply it — see that method's own doc.
//! - **Concurrent intake past the ring.** `exchange_step` itself still runs on
//!   one thread, draining and applying one command at a time — the ring is
//!   what lets *producers* genuinely race (see `exchange_inbound`'s own
//!   module doc); nothing requires `exchange_step`'s own apply loop to be
//!   concurrent too, and nothing here reads a clock, iterates a hash
//!   container, or compares an address regardless.

use std::collections::BTreeMap;

pub use exact_arith::
{
  Backing, ConservationError, Entry, KindError, MONEY_SCALE, Money, Quantity, Report, verify,
};
pub use exchange_book::{ Book, Resting };
pub use exchange_depth::{ Depth, DepthError, LevelView };
pub use exchange_escrow::{ Account, Conserved, Escrow, EscrowError, Holding };
pub use exchange_halt::HaltError;
pub use exchange_id::InstrumentId;
pub use exchange_inbound::
{
  BuildError, Consumer, Drain, Ends, InboundCmd, Producer, RingConfig, Split, inbound_flush, inbound_overflow_reject,
  inbound_ring,
};
use exchange_inbound::inbound_drain;
pub use exchange_match::{ Crossing, MatchError, SelfMatchCancellation, SelfMatchPolicy };
use exchange_rest::rest_place;
use exchange_seq::seq_next;
pub use exchange_snap::{ BookSnap, RestRow };
pub use exchange_spec::{ AssetId, InstrumentSpec, SpecError };
pub use exchange_stats::BookStats;
use exchange_stats::{ stats_cancel_add, stats_fill_add, stats_rest_add, stats_reject_add };
pub use exchange_tif::Tif;
use exchange_tif::tif_rests;
pub use exchange_types::
{
  AccountId, Amount, CancelCause, Event, EventKind, Obligation, Order, OrderId, Price, RejectReason,
  Sequence, Side, Trade, TypeError, notional, obligation,
};

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
  /// Whether `resting` is zero because the order's own [`Tif`] forbids
  /// resting an unfilled remainder (IOC drops it, FOK never partially fills
  /// in the first place) rather than because every unit found a
  /// counterparty. [`Self::is_complete`] uses this the same way it already
  /// uses [`Self::self_match_cancelled`].
  pub tif_dropped : bool,
}

impl Receipt
{
  /// Whether the order was entirely filled and nothing rests.
  ///
  /// Fix(is_complete_reported_a_cancellation_as_a_fill):
  /// Root cause: `resting` reaches zero two different ways — every unit
  /// found a counterparty, or the facade's step-4a self-match handling
  /// withdrew whatever was left instead of resting it — and this method
  /// returned `true` for
  /// both, contradicting its own "entirely filled" doc for the second: a
  /// self-match-cancelled order that traded nothing reported as complete.
  ///
  /// Pitfall: `resting == Quantity::ZERO` answers "is anything left to
  /// rest", not "did this fill" — the two coincide only when nothing was
  /// cancelled, so any later outcome that zeroes `resting` without a fill
  /// needs its own flag here too, not a widened zero-check.
  ///
  /// Fix(tif_forbidden_resting_was_not_tracked_as_its_own_reason):
  /// Root cause: the same zero-means-ambiguous trap this method was already
  /// fixed for once, recurring through a third path: an IOC/FOK order whose
  /// [`Tif`] forbids resting an unfilled remainder also zeroes `resting`,
  /// with no fill behind it, exactly like the self-match case above —
  /// caught before release by the new regression tests added alongside the
  /// facade's missing `tif_rests` gate (see `step_place`'s own fix), not by
  /// a user report.
  ///
  /// Pitfall: a disambiguating flag added for one cause of an ambiguous zero
  /// does not cover a second, later-discovered cause of the same zero — the
  /// fix pattern is "name the new cause its own flag too", not "trust the
  /// existing flag harder".
  #[ must_use ]
  pub fn is_complete( &self ) -> bool
  {
    self.resting == Quantity::ZERO && !self.self_match_cancelled && !self.tif_dropped
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
  /// [`Exchange::spec_register`] was asked about an instrument no spec names.
  UnknownInstrument( InstrumentId ),
  /// [`Exchange::spec_register`] refused a degenerate tick or lot.
  Spec( SpecError ),
  /// [`Exchange::spec_register`] was asked to register an instrument a
  /// previous call already registered — refused rather than overwritten, since
  /// silently changing a live grid would strand every order already resting
  /// against the old one.
  SpecAlreadyRegistered( InstrumentId ),
  /// [`Exchange::depth_get`] was asked for the top zero levels.
  Depth( DepthError ),
  /// [`Exchange::halt_set`]/[`Exchange::halt_clear`] asked for a state the
  /// instrument was already in.
  Halt( HaltError ),
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
      Self::UnknownInstrument( id ) => write!( f, "instrument {} has no registered spec", id.0 ),
      Self::Spec( error ) => write!( f, "spec refused: {error}" ),
      Self::SpecAlreadyRegistered( id ) => write!( f, "instrument {} is already registered", id.0 ),
      Self::Depth( DepthError::BadN ) => write!( f, "depth refused: the top zero levels has no answer" ),
      Self::Halt( HaltError::Already ) => write!( f, "halt refused: the instrument is already in that state" ),
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
      Self::Spec( error ) => Some( error ),
      Self::Rejected( _ ) | Self::NotResting( _ ) | Self::UnknownInstrument( _ )
      | Self::SpecAlreadyRegistered( _ ) | Self::Depth( _ ) | Self::Halt( _ ) => None,
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

impl From< SpecError > for ExchangeError
{
  fn from( error : SpecError ) -> Self
  {
    Self::Spec( error )
  }
}

impl From< DepthError > for ExchangeError
{
  fn from( error : DepthError ) -> Self
  {
    Self::Depth( error )
  }
}

impl From< HaltError > for ExchangeError
{
  fn from( error : HaltError ) -> Self
  {
    Self::Halt( error )
  }
}

/// What processing one drained [`InboundCmd`] produced.
///
/// `#[must_use]` on the type, not just the method returning a `Vec` of it —
/// an ignored rejection or an ignored not-yet-wired `Replace` are exactly the
/// silent failures the Contract's "no mutation without an event" rule exists
/// to make visible, and a caller that drops this value entirely still has
/// the event stream to fall back on, but should not do so by accident.
#[ must_use ]
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum StepOutcome
{
  /// An [`InboundCmd::Place`] ran the same validate-reserve-match-settle-rest
  /// pipeline the removed `submit` method used to run directly — see
  /// [`Exchange::exchange_step`]'s own doc for what changed.
  Placed( Result< Receipt, ExchangeError > ),
  /// An [`InboundCmd::Cancel`] ran through [`Exchange::cancel`] unchanged —
  /// see [`Exchange::exchange_step`]'s own doc for why this delegates rather
  /// than re-implementing cancellation.
  Cancelled( Result< Quantity, ExchangeError > ),
  /// An [`InboundCmd::Replace`] was drained, but this facade does not yet
  /// apply it — see [`Exchange::exchange_step`]'s own doc, "Replace is not
  /// yet wired" section.
  ReplaceNotWired,
}

/// One market: a book, the balances behind it, the record of everything that
/// happened, every instrument's own grid, and the running counters
/// [`Exchange::exchange_step`] keeps.
#[ derive( Debug, Clone, Default ) ]
pub struct Exchange
{
  book : Book,
  escrow : Escrow,
  next_order : u64,
  next_sequence : Sequence,
  events : Vec< Event >,
  specs : BTreeMap< InstrumentId, InstrumentSpec >,
  stats : BookStats,
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

  /// Register `id`'s grid and asset pair.
  ///
  /// # Errors
  ///
  /// [`ExchangeError::SpecAlreadyRegistered`] if `id` is already registered —
  /// refused rather than overwritten, since replacing a live tick/lot grid
  /// would silently change the terms under orders already resting against the
  /// old one. [`ExchangeError::Spec`] if `tick` or `lot` is zero.
  pub fn spec_register
  (
    &mut self,
    id : InstrumentId,
    base : AssetId,
    quote : AssetId,
    tick : Price,
    lot : Quantity,
  ) -> Result< (), ExchangeError >
  {
    if self.specs.contains_key( &id )
    {
      return Err( ExchangeError::SpecAlreadyRegistered( id ) );
    }
    let spec = exchange_spec::spec_new( id, base, quote, tick, lot )?;
    self.specs.insert( id, spec );
    Ok( () )
  }

  /// The top `n` price levels on each side of `instrument`'s book.
  ///
  /// Unlike [`Self::halt_set`], this needs no registered spec — it reads
  /// [`Book`] directly, the same as [`exchange_depth::depth_top`] itself.
  ///
  /// # Errors
  ///
  /// [`ExchangeError::Depth`] if `n` is zero.
  pub fn depth_get( &self, instrument : InstrumentId, n : usize ) -> Result< Depth, ExchangeError >
  {
    Ok( exchange_depth::depth_top( &self.book, instrument, n )? )
  }

  /// Halt matching on `instrument`. Resting orders are untouched.
  ///
  /// # Errors
  ///
  /// [`ExchangeError::UnknownInstrument`] if `instrument` was never
  /// registered via [`Self::spec_register`]. [`ExchangeError::Halt`] if it is
  /// already halted.
  pub fn halt_set( &mut self, instrument : InstrumentId ) -> Result< (), ExchangeError >
  {
    let spec = self.specs.get_mut( &instrument ).ok_or( ExchangeError::UnknownInstrument( instrument ) )?;
    Ok( exchange_halt::halt_set( spec )? )
  }

  /// Resume matching on `instrument`.
  ///
  /// # Errors
  ///
  /// [`ExchangeError::UnknownInstrument`] if `instrument` was never
  /// registered. [`ExchangeError::Halt`] if it was not halted.
  pub fn halt_clear( &mut self, instrument : InstrumentId ) -> Result< (), ExchangeError >
  {
    let spec = self.specs.get_mut( &instrument ).ok_or( ExchangeError::UnknownInstrument( instrument ) )?;
    Ok( exchange_halt::halt_clear( spec )? )
  }

  /// Whether `instrument` is currently halted.
  ///
  /// # Errors
  ///
  /// [`ExchangeError::UnknownInstrument`] if `instrument` was never
  /// registered.
  pub fn halt_is( &self, instrument : InstrumentId ) -> Result< bool, ExchangeError >
  {
    let spec = self.specs.get( &instrument ).ok_or( ExchangeError::UnknownInstrument( instrument ) )?;
    Ok( exchange_halt::halt_is( spec ) )
  }

  /// A point-in-time copy of `instrument`'s resting book, stamped with the
  /// caller-supplied `tick` — see [`exchange_snap::snap_take`] for why `tick`
  /// is never read from a clock here.
  #[ must_use ]
  pub fn snap_take( &self, instrument : InstrumentId, tick : Money ) -> BookSnap
  {
    exchange_snap::snap_take( &self.book, instrument, tick )
  }

  /// Take every event recorded so far, leaving the stream empty.
  ///
  /// The owned counterpart to [`Self::events`]'s borrowed slice — see
  /// `exchange_event`'s own module doc for why both exist.
  #[ must_use ]
  pub fn event_drain( &mut self ) -> Vec< Event >
  {
    exchange_event::event_drain( &mut self.events )
  }

  /// A copy of the running counters [`Self::exchange_step`] has kept so far.
  ///
  /// Counts only activity processed through [`Self::exchange_step`] —
  /// [`Self::cancel`]'s own direct, synchronous path is kept byte-for-byte
  /// unchanged by this stage's own scope, so it does not feed these counters.
  #[ must_use ]
  pub fn stats_get( &self ) -> BookStats
  {
    exchange_stats::stats_snapshot( &self.stats )
  }

  /// Drain `consumer` and apply every command it yields, in drain order, with
  /// `policy` governing self-match resolution for every
  /// [`InboundCmd::Place`] in the batch.
  ///
  /// # `exchange_step` owns sequencing
  ///
  /// A [`Resting`] inside an incoming [`InboundCmd::Place`] is a *draft*, not
  /// a finished order — its `order.id` and `arrival` are never trusted. This
  /// is the single-threaded apply side of the ring (see `exchange_inbound`'s
  /// own module doc, "Two producers without two threads on one ring"), so it
  /// is the only place with authoritative access to [`Self::claim_order`]'s
  /// counter and [`Self::emit`]'s sequence — exactly mirroring how the old
  /// `submit` never took a caller-supplied id either. Producers may push a
  /// placeholder id/arrival; `exchange_step` always overwrites both with its
  /// own, so global id-uniqueness and a correct, facade-authoritative total
  /// order hold regardless of what (if anything) a producer stamped on the
  /// way in.
  ///
  /// # `Cancel` delegates, it does not re-implement
  ///
  /// [`InboundCmd::Cancel`]'s own `instrument` field is never consulted —
  /// [`Self::cancel`] already does its own authoritative, instrument-agnostic
  /// search and is already correct and tested, so a command arriving via the
  /// ring gets exactly the same escrow-release behaviour a direct call would,
  /// with no second implementation to keep in sync.
  ///
  /// # Replace is not yet wired
  ///
  /// [`InboundCmd::Replace`] drains successfully but this method does not
  /// apply it — every drained `Replace` comes back as
  /// [`StepOutcome::ReplaceNotWired`]. A faithful replace has to move the old
  /// order's reservation and the new order's obligation atomically alongside
  /// the book-level swap [`exchange_rest::rest_replace`] already does, and
  /// report it with events whose `reserved`/`released` fields are honest —
  /// neither escrow orchestration nor a suitable event shape exists yet for
  /// that (`exchange_fill::EventKind` has no "replaced" kind, only
  /// `OrderAccepted`/`OrderCancelled`, each requiring the `Obligation` an
  /// escrow-free replace would have none of). Building that is new
  /// orchestration this stage was not asked for — see
  /// `exchange_rest`'s own module doc, which already names escrow
  /// orchestration as a facade-level concern it deliberately left open.
  /// Direct callers of `exchange_rest::rest_replace` are unaffected; so is
  /// `exchange_inbound`'s own test coverage of `InboundCmd::Replace` at the
  /// book level.
  pub fn exchange_step
  (
    &mut self,
    consumer : &mut Consumer< '_, InboundCmd >,
    policy : SelfMatchPolicy,
  ) -> Vec< StepOutcome >
  {
    inbound_drain( consumer ).into_iter().map( | cmd | self.step_one( cmd, policy ) ).collect()
  }

  fn step_one( &mut self, cmd : InboundCmd, policy : SelfMatchPolicy ) -> StepOutcome
  {
    match cmd
    {
      InboundCmd::Place( draft ) => StepOutcome::Placed( self.step_place( draft.order, policy ) ),
      InboundCmd::Cancel { id, .. } =>
      {
        let result = self.cancel( id );
        if result.is_ok() { stats_cancel_add( &mut self.stats, 1 ); }
        StepOutcome::Cancelled( result )
      },
      InboundCmd::Replace { .. } => StepOutcome::ReplaceNotWired,
    }
  }

  /// The old `submit`'s five-step body, generalized: `draft` supplies
  /// instrument/account/side/price/quantity/tif directly — no more pinning to
  /// a single instrument or `Tif::Gtc` — and `policy` is the caller's choice
  /// instead of a hardcoded `SelfMatchPolicy::CancelIncoming`. `draft.id` is
  /// never trusted; see [`Self::exchange_step`]'s own "owns sequencing"
  /// section.
  fn step_place( &mut self, draft : Order, policy : SelfMatchPolicy ) -> Result< Receipt, ExchangeError >
  {
    let order = Order { id : self.claim_order(), ..draft };

    // 1. Validate. Unchanged from the old `submit`'s own step 1 — see that
    // method's former `Fix(a_negative_price_is_refused_before_it_can_rest)`
    // comment (preserved in git history) for why the negative-price guard
    // lives here rather than inside escrow.
    if order.quantity == Quantity::ZERO
    {
      return Err( self.reject_counted( &order, RejectReason::ZeroQuantity ) );
    }
    if order.price < Money::ZERO
    {
      return Err( self.reject_counted( &order, RejectReason::NegativePrice ) );
    }
    // A halted instrument refuses every new placement outright — resting
    // orders are untouched (see `Self::halt_set`'s own doc), only new
    // arrivals are refused. No registered spec means no halt tracking
    // exists for this instrument, so an order against an unregistered
    // instrument validates exactly as it already did before `exchange_halt`
    // existed.
    //
    // Fix(halt_set_never_actually_blocked_a_placement):
    // Root cause: `Self::halt_set`/`halt_clear`/`halt_is` toggle
    // `InstrumentSpec::halted` correctly, but nothing in `step_place` ever
    // read it back — the flag was purely decorative from a placement's own
    // point of view. `exchange_halt`'s own Stage 7 work and this method's
    // Stage 9 rework landed independently, and nothing forced them to meet
    // until the wall smoke's own "halt, then place: refused" scenario tried
    // to exercise both together.
    //
    // Pitfall: a facade exposing a control (`halt_set`) is not the same
    // claim as the facade's own hot path consulting it — a crate whose own
    // tests all pass can still be wired to nothing.
    if self.specs.get( &order.instrument ).is_some_and( exchange_halt::halt_is )
    {
      return Err( self.reject_counted( &order, RejectReason::Halted ) );
    }

    // 2. Dry-run the whole operation — reserve, cross, settle every trade,
    // release every self-match cancellation — against scratch clones, before
    // any of it touches real state. Unchanged from the old `submit`'s own
    // step 2 — see that method's former
    // `Fix(exchange_submit_partial_crossing_could_strand_state)` comment
    // (preserved in git history) for why.
    let mut dry_escrow = self.escrow.clone();
    if let Err( error ) = dry_escrow.reserve( &order )
    {
      return Err( self.reject_counted( &order, Self::reason_for( error ) ) );
    }
    let mut dry_book = self.book.clone();
    let dry_crossing = exchange_match::cross( &mut dry_book, &order, policy )?;
    for trade in &dry_crossing.trades
    {
      if let Err( error ) = dry_escrow.settle( trade, order.side, order.price )
      {
        return Err( self.reject_counted( &order, Self::reason_for( error ) ) );
      }
    }
    for cancellation in &dry_crossing.cancelled
    {
      if let Err( error ) = dry_escrow.release( cancellation.account, cancellation.order )
      {
        return Err( self.reject_counted( &order, Self::reason_for( error ) ) );
      }
    }

    // 3. The dry run above succeeded in full — replay it for real. Every
    // call below is guaranteed to succeed: same starting state, the same
    // pure `cross`, and nothing else touches `self.escrow`/`self.book` in
    // between.
    let reserved = self.escrow.reserve( &order ).expect( "already validated by the dry run above" );
    let arrival = self.emit
    (
      order.id, order.account,
      EventKind::OrderAccepted { side : order.side, price : order.price, quantity : order.quantity, reserved },
    );
    let crossing = exchange_match::cross( &mut self.book, &order, policy )
      .expect( "already produced by the identical dry run above — cross is pure" );

    // 4. Settle each trade in the step that generated it.
    stats_fill_add( &mut self.stats, crossing.trades.len() as u64 );
    for trade in &crossing.trades
    {
      self.escrow.settle( trade, order.side, order.price ).expect( "already validated by the dry run above" );
      self.emit( order.id, order.account, EventKind::Trade( *trade ) );
    }

    // 4a. Release and report each self-match cancellation.
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

    // 5. Rest the remainder, reservation retained — via `exchange_rest::rest_place`
    // now that this facade consumes that crate, rather than reaching into
    // `self.book.insert` directly. `tif_rests` gates this the same way
    // `demo_p22_ioc`'s own direct orchestration already does: `cross` never
    // inserts a remainder for any TIF by its own design (see that crate's
    // module doc), so whether one ever reaches the book is entirely this
    // caller's decision, and an IOC/FOK order's unfilled remainder must
    // never make it to `rest_place`.
    //
    // Fix(tif_requires_full_orders_were_rested_instead_of_rejected):
    // Root cause: an unfillable FOK comes back from `cross` shaped exactly
    // like an ordinary no-cross outcome — empty trades, full remaining,
    // `Ok`, per that crate's own module doc, which says in so many words
    // that translating "FOK, nothing filled" into a rejection is this
    // facade's job. `step_place` never did that translation, nor did it
    // consult `tif_rests` for IOC's own partial-fill remainder — both TIFs
    // fell through to the same unconditional `resting = crossing.remaining`
    // the old `submit` used when every order was hardcoded `Tif::Gtc`, so a
    // FOK against a thin book rested the whole quantity instead of being
    // refused, and an IOC's unfilled remainder rested instead of dropping.
    // Caught by the new `an_ioc_taker_never_rests_its_remainder_through_the_facade`/
    // `a_fok_taker_rejects_whole_against_a_thin_book_through_the_facade` tests,
    // not by a user report.
    //
    // Pitfall: `exchange_match::cross`'s own module doc already states that
    // IOC/FOK remainder disposal is the caller's decision, in a crate this
    // facade depends on directly — stating the obligation in the crate that
    // does not do it is not the same as the crate that must do it actually
    // doing it, and nothing short of a TIF-specific regression test catches
    // the gap between the two.
    let tif_dropped = !incoming_cancelled && !tif_rests( order.tif ) && crossing.remaining > Quantity::ZERO;

    // 4b. A TIF-dropped remainder must release its own reservation now —
    // nothing else will, since step 5 below is exactly what stops it from
    // resting, and resting is the only other thing that keeps a reservation
    // alive past this call.
    //
    // Fix(tif_dropped_remainder_leaked_its_own_reservation):
    // Root cause: `Escrow::reserve` reserves an order's full notional up
    // front, and `Escrow::settle` only ever reduces that reservation by
    // whatever quantity actually filled (see `reduced_obligation`) — neither
    // call releases the rest. Ordinarily the unfilled rest stays reserved
    // because the order rests, and a later cancel or fill releases it then.
    // Once step 5 stopped resting an IOC/FOK remainder (the fix directly
    // above this one), nothing was left to ever call `Escrow::release` for
    // it: not resting, not self-match cancellation (`crossing.cancelled`
    // only ever names a self-match pair, never an ordinary TIF disposal),
    // not a future cancel (there is nothing on the book to cancel). The
    // reservation would have stayed on `self.escrow`'s books forever —
    // exactly the leaked-reservation failure
    // `docs/invariant/001_escrow_covers_resting_orders.md` names as a silent
    // deletion of wealth, caught here before release rather than by a later
    // conservation-audit imbalance with no event to trace it to.
    //
    // Pitfall: fixing "don't rest this" in isolation, without asking what
    // used to keep the reservation this remainder depends on alive, trades
    // one invariant violation (wrongly visible on the book) for a worse one
    // (invisible everywhere). A resource a state machine releases on every
    // other exit path needs the same release on a newly-added exit path too.
    if tif_dropped
    {
      let released = self.escrow.release( order.account, order.id )
        .expect( "settle reduced this order's own reservation to exactly its unfilled remainder, never removing the entry" );
      self.emit
      (
        order.id, order.account,
        EventKind::OrderCancelled { cause : CancelCause::TimeInForce, quantity : crossing.remaining, released },
      );
    }

    // 5. Rest the remainder, reservation retained — via `exchange_rest::rest_place`
    // now that this facade consumes that crate, rather than reaching into
    // `self.book.insert` directly. `tif_rests` gates this the same way
    // `demo_p22_ioc`'s own direct orchestration already does: `cross` never
    // inserts a remainder for any TIF by its own design (see that crate's
    // module doc), so whether one ever reaches the book is entirely this
    // caller's decision, and an IOC/FOK order's unfilled remainder must
    // never make it to `rest_place` — see the two fixes immediately above
    // for why, and for the dry-run mirror of this same gate.
    let resting = if incoming_cancelled || tif_dropped { Quantity::ZERO } else { crossing.remaining };
    if resting > Quantity::ZERO
    {
      assert!
      (
        rest_place( &mut self.book, Resting { order, remaining : resting, arrival } ),
        "order id came from this exchange's own next_order counter, which never repeats",
      );
      stats_rest_add( &mut self.stats, 1 );
    }

    Ok( Receipt { order : order.id, trades : crossing.trades, resting, self_match_cancelled : incoming_cancelled, tif_dropped } )
  }

  /// [`Self::reject`], plus the stats counter [`Self::exchange_step`] keeps.
  fn reject_counted( &mut self, order : &Order, reason : RejectReason ) -> ExchangeError
  {
    stats_reject_add( &mut self.stats, 1 );
    self.reject( order, reason )
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

  /// Map what `Escrow::reserve` refused, in step 2 of the old `submit`
  /// (now `step_place`'s own step 2), to the reason reported to the caller.
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
