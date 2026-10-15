//! The self-trade policy: how a candidate pair sharing one account resolves.
//!
//! A root of the dependency tree: no dependency on any other `exchange_*`
//! crate, and this crate does not itself walk the book — it names the closed
//! set of three policies `exchange_match` consults on every candidate pair.
//! Without a named policy, a self-match either fills by accident or is
//! resolved by convention nobody wrote down.
//!
//! `SelfMatchPolicy` moved here verbatim from `exchange_match`, which still
//! re-exports it so existing callers are unaffected.
//!
//! # Not built: `Allow`
//!
//! The source design names three policies, including an `Allow` that lets a
//! self-match cross like any other pair. The real design never had one: a
//! self-match is refused unconditionally, by construction, before any trade
//! for the pair is built — see `exchange_match`'s own module documentation,
//! "Self-match prevention is ... fixed, never a mode." Adding `Allow` would
//! not be an extraction, it would be new, unrequested permission for
//! accounts to trade with themselves, so this crate keeps the real three:
//! [`SelfMatchPolicy::CancelResting`], [`SelfMatchPolicy::CancelIncoming`],
//! [`SelfMatchPolicy::CancelBoth`]. The first two correspond exactly to the
//! source's `CancelOldest`/`CancelNewest` — in a self-match the resting order
//! is always the older one and the incoming order is always the newer one,
//! so "cancel resting" and "cancel oldest" withdraw the same order; the real
//! names describe the order's *role* in the cross rather than its arrival
//! time. `CancelBoth` has no counterpart in the source's three.

/// One of the three ways a candidate pair sharing one account resolves.
///
/// Configured per book, never per order — a submitter is never handed a
/// choice of which side dies.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum SelfMatchPolicy
{
  /// The resting order is withdrawn; the incoming order's match loop resumes
  /// at whatever is now the front of the book. Equivalent in effect to the
  /// source design's `CancelOldest` — see the module documentation.
  CancelResting,
  /// The incoming order's remainder is withdrawn; the resting order survives
  /// untouched. Equivalent in effect to the source design's `CancelNewest`.
  CancelIncoming,
  /// Both remainders are withdrawn. No counterpart in the source's three.
  CancelBoth,
}

/// A short, stable name for `policy` — the source design's `stp_name`.
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
