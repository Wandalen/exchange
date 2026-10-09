//! Ring-fed inbound commands: many producer lanes, one deterministic apply.
//!
//! Closes hard problems 11 (inbound at the aeon edge), 23 (full order of
//! inbound) and 24 (ring overflow is reject), and features 29 (inbound
//! flush, drain, then rest or match) and 30 (overflow → reject) — not hard
//! problem 1, which is `exchange_book`'s own "one book per instrument" with
//! no producer-ordering content. [`inbound_drain`] is the already-drained
//! slice hard problem 11 asks for; the fixed lane order the "Two producers"
//! section below describes is hard problem 23's stable total order;
//! [`inbound_overflow_reject`]/[`ring_types::OverflowPolicy::Fail`] is hard
//! problem 24 and feature 30 together; [`inbound_flush`] →
//! [`inbound_drain`] → [`inbound_apply`] is feature 29's flush-drain-then-
//! rest-or-match sequence end to end. This crate is the family's first use
//! of real concurrency. Depends on
//! `ring_factory` + `ring_handle` + `ring_types` only, per
//! `docs/ring_edge/001_*.md`'s resolved finding: `ring_handle::Producer`
//! cannot be cloned (by design — see its own module doc's "`try_clone` is
//! withheld" row) and `ring_core`/`ring_mpsc`'s actual multi-producer claim
//! path sits below the family's five-crate export Contract. One ring,
//! reached through this crate, therefore has **exactly one producer**, full
//! stop — never two threads racing on the same `Producer` value.
//!
//! # Two producers without two threads on one ring
//!
//! `docs/phase/028_p28_drain.md` asks for drain order to be deterministic
//! "not thread-completion-order-dependent" across **real** concurrent
//! producers — so simulating "two producers" as one thread pushing twice,
//! sequentially, would not exercise the property the phase cares about at
//! all. The resolution: **one [`inbound_ring`] per producer**, each with its
//! own exclusively-owned [`Producer`], genuinely raced from its own OS
//! thread — and a consumer-side combine step that concatenates each lane's
//! [`inbound_drain`] result in a **fixed lane order decided ahead of time**,
//! never by which thread happened to finish first. Real concurrency at the
//! push side; a result insensitive to its scheduling at the drain side. See
//! `docs/decisions/001_two_producers_is_two_rings.md` for the full case
//! against the alternative (one shared ring, `Producer` behind a `Mutex`) —
//! that would reintroduce the exact lock contention this family exists to
//! avoid, for a capability (`ring_handle::Producer::try_clone`) this crate's
//! dependency set does not have anyway.
//!
//! # Overflow is a reject, not a drop — by explicit configuration
//!
//! `RingConfig::new`'s own default overflow policy is
//! [`ring_types::OverflowPolicy::DropNewest`] — a full ring's `try_push`
//! would return `Ok(())` while silently discarding the record (see
//! `ring_handle::Producer::try_push`'s own doc comment: "an `Ok` is not by
//! itself evidence the record was kept"). `docs/phase/029_p29_over.md`
//! wants the opposite: a full ring must come back as a **visible** rejection.
//! [`inbound_ring`] therefore always builds with
//! [`ring_types::OverflowPolicy::Fail`] — never the default — which is what
//! makes [`inbound_overflow_reject`]'s `Err` case reachable at all rather
//! than vacuously unreachable behind a policy that never fails.
//!
//! # `inbound_apply`'s `Place` reaches into `exchange_match`, not just `exchange_rest`
//!
//! A `Place` command names a *new* order, not one already resting — crossing
//! it against the book before any remainder rests is what
//! `exchange_match::cross` exists for, and skipping that step (going
//! straight to `exchange_rest::rest_place`, an unconditional insert) would
//! let a marketable order rest unmatched, silently wrong. `Cancel`/`Replace`
//! act on an order already on the book, which is exactly `exchange_rest`'s
//! own scope — no crossing involved either way. This mirrors
//! `exchange_core::submit`'s shape in miniature, without duplicating its
//! cap/escrow orchestration: those stay facade-level concerns Stage 9 has
//! not yet wired this crate into. Idempotency is the one exception — see
//! the next section for why it is checked here instead.
//!
//! # Idempotency
//!
//! [`inbound_apply`] takes a `seen : &mut IdSet`, refuses a
//! [`InboundCmd::Place`] whose id it already claims before crossing, and
//! claims the id with `exchange_idem::idem_insert` once the remainder rests —
//! and un-claims it with
//! `exchange_idem::idem_remove` once a [`InboundCmd::Cancel`]/
//! [`InboundCmd::Replace`] actually withdraws it, so a legitimate
//! cancel-then-resubmit is accepted again rather than permanently refused.
//!
//! This is *not* the same orchestration-stays-in-the-facade story the
//! paragraph above tells for cap/escrow. Unlike those two, idempotency has a
//! real, demonstrated gap at this crate's own level today: `exchange_core`'s
//! facade never reaches `inbound_apply` at all (`Exchange::step_one` runs
//! its own, separate, already-id-fresh pipeline — see that method's own
//! doc), so an id arriving through *this* crate's own public
//! [`inbound_apply`] has no upstream guarantee of freshness the way one
//! arriving through the facade does. Before this check existed, a repeat id
//! reaching [`rest_place`]'s own pre-existing duplicate guard was silently
//! dropped in release builds — see [`inbound_apply`]'s own
//! `Fix(exchange_inbound/BUG-003)` comment.
//!
//! See `exchange_idem`'s own module doc for why this does not duplicate
//! `exchange_book::Book::insert`'s pre-existing duplicate-id guard: that
//! guard already *has* the same information, just without a name a caller
//! can branch on separately from "this quantity was already zero" — this
//! crate's `idem_insert` call is what gives it one, named and returned to
//! the caller rather than silently discarded the way a bare `rest_place`
//! bool was before `Fix(exchange_inbound/BUG-003)`. An id is claimed only
//! while its order rests, but checked before crossing: a repeat that crossed
//! first would trade against the book and then be refused.
//!
//! # What this crate does not do
//!
//! It does not assign `arrival`/`Sequence` — a [`Resting`] inside
//! [`InboundCmd::Place`] arrives already sequenced, by whichever producer
//! built it (realistically, the eventual facade's `seq_next()`, once Stage 9
//! wires this crate in). Sequencing inbound commands is an accept-time
//! decision; this crate only carries and applies what it is given.

use exchange_book::{ Book, Resting };
use exchange_id::{ InstrumentId, OrderId };
use exchange_idem::{ idem_insert, idem_remove, idem_seen };
use exchange_match::{ cross, Crossing, MatchError, SelfMatchPolicy };
use exchange_rest::{ rest_cancel, rest_place, rest_replace, RestReplaceError };
use exchange_tif::tif_rests;
use ring_factory::Factory;
use ring_types::OverflowPolicy;

pub use exchange_idem::{ IdemError, IdSet };
pub use ring_factory::{ BuildError, RingConfig };
pub use ring_handle::{ Consumer, Drain, Ends, Producer, Split };

/// One command carried from a producer, through the ring, to the book.
///
/// Every variant is a direct call shape for one of `exchange_rest`'s three
/// non-matching moves, or — for [`Place`](Self::Place) — for
/// `exchange_match::cross` instead, per the module doc's "`inbound_apply`'s
/// `Place`" section.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum InboundCmd
{
  /// A new order, not yet on the book. Carries its own `remaining`/`arrival`
  /// — see the module doc's "What this crate does not do".
  Place( Resting ),
  /// Withdraw `id`, resting on `instrument`'s book, whichever side.
  Cancel
  {
    /// Which instrument's book `id` rests on.
    instrument : InstrumentId,
    /// The resting order to withdraw.
    id : OrderId,
  },
  /// Atomically replace `old_id` with `new_resting` — see
  /// [`exchange_rest::rest_replace`] for the all-or-nothing guarantee.
  Replace
  {
    /// Which instrument's book `old_id` rests on.
    instrument : InstrumentId,
    /// The resting order being replaced.
    old_id : OrderId,
    /// What replaces it, if `old_id` is found.
    new_resting : Resting,
  },
}

/// What applying one [`InboundCmd`] produced.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum InboundOutcome
{
  /// [`InboundCmd::Place`] was crossed against the book.
  Crossed( Crossing ),
  /// [`InboundCmd::Cancel`] ran; [`None`] if nothing was resting under that id.
  Cancelled( Option< Resting > ),
  /// [`InboundCmd::Replace`] ran; see [`RestReplaceError`] for the failure shapes.
  Replaced( Result< Resting, RestReplaceError > ),
}

/// Why [`inbound_apply`] could not apply a drained command.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum InboundApplyError
{
  /// [`exchange_match::cross`] itself refused. Reachable only as
  /// [`MatchError::PostOnlyWouldTake`]; see that function's own doc for why
  /// every other variant is not.
  Match( MatchError ),
  /// [`InboundCmd::Place`] named an id that is already claimed by an earlier,
  /// not-yet-cancelled `Place` — a retried submission, not a new order. See
  /// the module doc's "Idempotency" section.
  Idem( IdemError ),
}

impl core::fmt::Display for InboundApplyError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::Match( error ) => write!( f, "matching failed: {error}" ),
      Self::Idem( error ) => write!( f, "place refused: {error}" ),
    }
  }
}

impl core::error::Error for InboundApplyError
{
  fn source( &self ) -> Option< &( dyn core::error::Error + 'static ) >
  {
    match self
    {
      Self::Match( error ) => Some( error ),
      Self::Idem( error ) => Some( error ),
    }
  }
}

impl From< MatchError > for InboundApplyError
{
  fn from( error : MatchError ) -> Self
  {
    Self::Match( error )
  }
}

impl From< IdemError > for InboundApplyError
{
  fn from( error : IdemError ) -> Self
  {
    Self::Idem( error )
  }
}

/// Build one ring of `capacity` inbound-command slots.
///
/// Always configured with [`OverflowPolicy::Fail`] — never the ring family's
/// own default — per the module doc's "Overflow is a reject" section. One
/// ring is one producer; see the module doc's "Two producers without two
/// threads" section for why many producers means many rings, not one shared
/// one.
///
/// # Errors
///
/// [`BuildError`] on an invalid `capacity` (zero, or not a power of two) or
/// an unsupported configuration — neither reachable here, since the only
/// policy ever requested is one every in-house backend accepts.
pub fn inbound_ring( capacity : usize ) -> Result< Split< InboundCmd >, BuildError >
{
  let cfg = RingConfig::new( capacity ).map_err( BuildError::Unsupported )?.with_overflow( OverflowPolicy::Fail );
  Factory.build::< InboundCmd >( cfg )
}

/// Publish as many of `cmds` as the ring will take.
///
/// A thin forward to [`Producer::try_push_batch`] — see that method's own
/// doc comment: partial acceptance is the normal case, not a failure.
// Fix(exchange_inbound/BUG-001): try_push_batch returns Result<usize, (usize, T)>,
// not usize — the Err case still carries the accepted count (the usize field),
// which is what a partial-acceptance caller needs, same as the Ok case.
// Root cause: inbound_flush was written against an assumed usize-returning
// signature; ring_handle::Producer::try_push_batch's real signature already
// returns Result<usize, (usize, T)> and was never usize — this forward never
// compiled against the real API.
// Pitfall: a "thin forward" doc comment is not itself proof the call compiles —
// confirm the wrapped method's real signature, not just its documented intent.
pub fn inbound_flush( producer : &mut Producer< '_, InboundCmd >, cmds : impl IntoIterator< Item = InboundCmd > ) -> usize
{
  let mut cmds = cmds.into_iter();
  match producer.try_push_batch( &mut cmds )
  {
    Ok( n ) | Err( ( n, _ ) ) => n,
  }
}

/// Publish one command, surfacing a full ring as `Err` rather than a silent
/// drop.
///
/// Reachable only because [`inbound_ring`] always builds with
/// [`OverflowPolicy::Fail`] — on any other policy this can only return `Ok`,
/// policy-dependent silence the module doc's "Overflow is a reject" section
/// names directly.
///
/// # Errors
///
/// Hands `cmd` back, unpublished, when the ring has no free slot.
pub fn inbound_overflow_reject( producer : &mut Producer< '_, InboundCmd >, cmd : InboundCmd ) -> Result< (), InboundCmd >
{
  producer.try_push( cmd )
}

/// Take every command published before this call, as a plain `Vec`.
///
/// A thin forward to [`Consumer::drain`] — already bounded at the call that
/// makes it, so this terminates under a still-live producer.
pub fn inbound_drain( consumer : &mut Consumer< '_, InboundCmd > ) -> Vec< InboundCmd >
{
  consumer.drain().collect()
}

/// Apply one drained [`InboundCmd`] to `book`, tracking claimed ids in `seen`.
///
/// `policy` is supplied by the caller on every call, same as
/// `exchange_match::cross` itself — this crate holds no state of its own
/// between calls either; `seen` is the caller's, threaded through exactly
/// like `book`. See the module doc's "`inbound_apply`'s `Place`" section for
/// why [`InboundCmd::Place`] reaches `exchange_match` rather than
/// `exchange_rest::rest_place` directly, and its "Idempotency" section for
/// why `seen` exists at all.
///
/// # Errors
///
/// [`InboundApplyError::Match`] exactly when `exchange_match::cross` itself
/// returns one — in practice a post-only order that would take, with `book`
/// untouched.
/// [`InboundApplyError::Idem`] if [`InboundCmd::Place`] names an id `seen`
/// already claims, with `book` untouched.
pub fn inbound_apply( book : &mut Book, seen : &mut IdSet, policy : SelfMatchPolicy, cmd : InboundCmd ) -> Result< InboundOutcome, InboundApplyError >
{
  match cmd
  {
    InboundCmd::Place( resting ) =>
    {
      // Fix(inbound_apply_duplicate_traded_before_refusal): a repeated id was
      // checked only where its remainder would rest — after `cross` had
      // already traded it against the book — so the caller got `Err` and
      // lost trades that had happened.
      // Root cause: the check sat where the harm was assumed to be (a second
      // rest), not where the book is first touched.
      // Pitfall: refuse before the first mutation, not before the last one.
      if idem_seen( seen, resting.order.id )
      {
        return Err( InboundApplyError::Idem( IdemError::Duplicate ) );
      }
      let crossing = cross( book, &resting.order, policy )?;

      // Fix(exchange_inbound/BUG-002): a self-match-cancelled incoming order was
      // rested anyway, because `!crossing.is_complete()` is true whenever
      // `crossing.remaining > 0` regardless of *why* — the self-match branch
      // (`SelfMatchPolicy::CancelIncoming`/`CancelBoth`) leaves the full
      // cancelled quantity in `remaining` on purpose (see `Crossing::remaining`'s
      // own doc comment), which this code read as an ordinary unfilled
      // remainder instead of a cancellation. `exchange_core::step_place`
      // already solved this exact problem (`incoming_cancelled`, scanning
      // `crossing.cancelled` for the incoming order's own id) — ported here.
      // Root cause: `crossing.is_complete()`/`crossing.remaining` alone cannot
      // distinguish "nothing left to fill" from "cancelled, not meant to rest"
      // — only `crossing.cancelled` carries that distinction.
      // Pitfall: `remaining > 0` means "not finished", not "safe to rest" —
      // check `cancelled` for the incoming order's own id before resting
      // anything derived from `remaining`.
      let incoming_cancelled = crossing.cancelled.iter().any( | c | c.order == resting.order.id );

      if !incoming_cancelled && !crossing.is_complete() && tif_rests( resting.order.tif )
      {
        // Fix(exchange_inbound/BUG-003): a duplicate id reaching this point was
        // silently dropped in release builds — `rest_place`'s own `false` was
        // only ever checked by a `debug_assert!`, compiled out entirely outside
        // debug, so the remainder neither rested nor was reported, with no event
        // stream here to fall back on the way `exchange_core::step_place` has.
        // Root cause: this function trusted `resting.order.id` to be fresh
        // because `exchange_core`'s own pipeline guarantees that — true for the
        // one caller Stage 9 actually wired up, but `inbound_apply` is a public
        // function nothing stops a different caller from invoking directly with
        // a hand-built, not-necessarily-fresh id (see this module's own
        // "Idempotency" section).
        // Pitfall: a `debug_assert!` guarding an invariant that only holds for
        // today's one caller is not a check — it is a release-mode no-op with a
        // comment attached. An invariant a *public* function depends on must be
        // enforced for every caller, not assumed from the one caller that
        // currently happens to satisfy it.
        idem_insert( seen, resting.order.id ).expect( "checked unseen before crossing" );
        let remainder = Resting { remaining : crossing.remaining, ..resting };
        let placed = rest_place( book, remainder );
        debug_assert!( placed, "idem_insert above already refused a repeat; a freshly-claimed id cannot also collide in Book::insert" );
      }

      Ok( InboundOutcome::Crossed( crossing ) )
    },
    InboundCmd::Cancel { instrument, id } =>
    {
      let cancelled = rest_cancel( book, instrument, id );
      if cancelled.is_some()
      {
        idem_remove( seen, id );
      }
      Ok( InboundOutcome::Cancelled( cancelled ) )
    },
    InboundCmd::Replace { instrument, old_id, new_resting } =>
    {
      let new_id = new_resting.order.id;
      let result = rest_replace( book, instrument, old_id, new_resting );
      if result.is_ok()
      {
        idem_remove( seen, old_id );
        idem_insert( seen, new_id )
          .expect( "rest_replace's own underlying Book::insert already proved this id free" );
      }
      Ok( InboundOutcome::Replaced( result ) )
    },
  }
}
