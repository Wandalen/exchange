//! The self-trade policy: how a candidate pair sharing one account resolves.
//!
//! A root of the dependency tree. This crate does not walk the book; it names
//! the policies `exchange_match` applies to every candidate pair, and
//! `exchange_match` re-exports [`SelfMatchPolicy`].
//!
//! # Not built: `Allow`
//!
//! A self-match is always refused — `exchange_match` treats that as fixed,
//! never a mode — so the source design's `Allow` has nothing to select. Its
//! `CancelOldest`/`CancelNewest` are named by role instead: in a self-match
//! the resting order is always the older one, so
//! [`SelfMatchPolicy::CancelResting`] withdraws the same order `CancelOldest`
//! would, and [`SelfMatchPolicy::CancelIncoming`] the same as `CancelNewest`.
//! [`SelfMatchPolicy::CancelBoth`] has no source counterpart. See
//! `docs/decisions/001_no_allow_resting_incoming_naming.md`.

/// How a candidate pair sharing one account resolves.
///
/// Chosen by the operator, never by the submitter:
/// `exchange_core::Exchange::exchange_step` takes one policy for a whole drain.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum SelfMatchPolicy
{
  /// The resting order is withdrawn; the incoming order keeps matching
  /// against the new front of the book. The source design's `CancelOldest`.
  CancelResting,
  /// The incoming order's remainder is withdrawn; the resting order is
  /// untouched. The source design's `CancelNewest`.
  CancelIncoming,
  /// Both remainders are withdrawn. No source counterpart.
  CancelBoth,
}

/// A stable `snake_case` name for `policy` — the source design's `stp_name`.
#[ must_use ]
pub const fn stp_name( policy : SelfMatchPolicy ) -> &'static str
{
  match policy
  {
    SelfMatchPolicy::CancelResting => "cancel_resting",
    SelfMatchPolicy::CancelIncoming => "cancel_incoming",
    SelfMatchPolicy::CancelBoth => "cancel_both",
  }
}
