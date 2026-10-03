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
//! [`docs/phase/028_p28_drain.md`] asks for drain order to be deterministic
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
//! itself evidence the record was kept"). [`docs/phase/029_p29_over.md`]
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
//! idempotency/cap/escrow orchestration: those stay facade-level concerns
//! Stage 9 has not yet wired this crate into.
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
use exchange_match::{ cross, Crossing, MatchError, SelfMatchPolicy };
use exchange_rest::{ rest_cancel, rest_place, rest_replace, RestReplaceError };
use exchange_tif::tif_rests;
use ring_factory::Factory;
use ring_types::OverflowPolicy;

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
pub fn inbound_flush( producer : &mut Producer< '_, InboundCmd >, cmds : impl IntoIterator< Item = InboundCmd > ) -> usize
{
  let mut cmds = cmds.into_iter();
  producer.try_push_batch( &mut cmds )
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

/// Apply one drained [`InboundCmd`] to `book`.
///
/// `policy` is supplied by the caller on every call, same as
/// `exchange_match::cross` itself — this crate holds no state between calls
/// either. See the module doc's "`inbound_apply`'s `Place`" section for why
/// [`InboundCmd::Place`] reaches `exchange_match` rather than
/// `exchange_rest::rest_place` directly.
///
/// # Errors
///
/// [`MatchError`] exactly when `exchange_match::cross` itself would return
/// one — unreachable through any path this crate exercises, same as that
/// function's own documented guarantee.
pub fn inbound_apply( book : &mut Book, policy : SelfMatchPolicy, cmd : InboundCmd ) -> Result< InboundOutcome, MatchError >
{
  match cmd
  {
    InboundCmd::Place( resting ) =>
    {
      let crossing = cross( book, &resting.order, policy )?;

      if !crossing.is_complete() && tif_rests( resting.order.tif )
      {
        let remainder = Resting { remaining : crossing.remaining, ..resting };
        let placed = rest_place( book, remainder );
        debug_assert!( placed, "a GTC remainder under the incoming order's own just-submitted id cannot already rest elsewhere" );
      }

      Ok( InboundOutcome::Crossed( crossing ) )
    },
    InboundCmd::Cancel { instrument, id } => Ok( InboundOutcome::Cancelled( rest_cancel( book, instrument, id ) ) ),
    InboundCmd::Replace { instrument, old_id, new_resting } =>
      Ok( InboundOutcome::Replaced( rest_replace( book, instrument, old_id, new_resting ) ) ),
  }
}
