//! A monotonic sequence for time priority, without a wall clock.
//!
//! A root of the dependency tree: no dependency on any other `exchange_*`
//! crate, and not workstream 004's clock. Without a dedicated sequence type,
//! an `Instant` ends up stamping the book directly, which breaks determinism
//! and replay — two orders submitted in the same instant would need their tie
//! broken by a clock reading, and that reading disagrees across hosts.
//!
//! `Sequence` moved here verbatim from `exchange_types`, which still
//! re-exports it so existing callers are unaffected. [`seq_next`] is new: it
//! gives the increment a name and a home, replacing the raw `u64` counter
//! `exchange_core` previously incremented by hand.
//!
//! # Not built: `SeqError::Exhausted`
//!
//! The source design names an error for a sequence that overflows its
//! backing width. At one increment per accepted order, exhausting a `u64`
//! takes longer than any run of this exchange will ever last — modelling a
//! failure mode nothing can reach is a check with no test that could ever
//! legitimately fail it, so it is left out rather than shipped unreachable.
//!
//! # Not built: `seq_cmp`
//!
//! [`Sequence`] already derives `Ord`; a free function wrapping `.cmp()`
//! would be the same comparison under a second name — exactly the "two
//! sources of truth for one fact" `exchange_types` warns against for `Trade`
//! versus `Fill`, for the same reason.

/// A position in the total order of events.
///
/// Claimed, never measured. It is not a clock reading, and no matching
/// decision may consult one: two orders submitted in the same instant still
/// receive distinct, ordered positions, and a rule that broke the tie by
/// timestamp would resolve it differently on a host whose clock disagreed.
#[ derive( Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct Sequence( pub u64 );

impl Sequence
{
  /// The sequence's starting value — the source design's `seq_zero`.
  pub const ZERO : Self = Self( 0 );
}

/// The next sequence after `current` — this crate's one piece of behaviour.
///
/// A free function, named per the source design's noun-verb convention,
/// rather than a method: `exchange_core` holds the current value and calls
/// this once per accepted order, which is the entire contract a caller needs.
#[ must_use ]
pub const fn seq_next( current : Sequence ) -> Sequence
{
  Sequence( current.0 + 1 )
}
