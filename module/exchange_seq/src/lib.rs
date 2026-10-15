//! A monotonic sequence for time priority, without a wall clock.
//!
//! A root of the dependency tree, and not workstream 004's clock. An
//! `Instant` on the book would break determinism and replay: two orders in
//! the same instant would need a clock reading to break the tie, and clocks
//! disagree across hosts.
//!
//! # Not built: `seq_cmp`, `SeqError::Exhausted`
//!
//! [`Sequence`] derives `Ord`, so `seq_cmp` would be `.cmp()` under a second
//! name. A `u64` bumped once per event does not run out in any real run, so
//! [`seq_next`] stays infallible. See
//! `docs/decisions/001_no_seq_cmp_or_seq_error.md`.

/// A position in the total order of events.
///
/// Claimed, never measured: two orders submitted in the same instant still
/// get distinct, ordered positions, and no matching decision reads a clock.
#[ derive( Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct Sequence( pub u64 );

impl Sequence
{
  /// The starting value — the source design's `seq_zero`.
  pub const ZERO : Self = Self( 0 );
}

/// The position after `current`. `exchange_core` calls this once per emitted
/// event.
#[ must_use ]
pub const fn seq_next( current : Sequence ) -> Sequence
{
  Sequence( current.0 + 1 )
}
