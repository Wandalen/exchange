//! Running counters for the hot path: rests, fills, rejects and cancels.
//!
//! A root of the dependency tree. `exchange_core` bumps the counters on every
//! step, per instrument and in total, and serves them through
//! `Exchange::stats_get`/`stats_get_for`, so a caller reads a number instead
//! of re-scanning the event log.
//!
//! The source design lists `exchange_id` as a dependency; no counter is keyed
//! by an id, so it is not taken — see
//! `docs/decisions/001_no_exchange_id_dependency.md`. Its function list also
//! has no incrementer for `rests` or `cancels`; [`stats_rest_add`] and
//! [`stats_cancel_add`] fill that gap.

/// Running counts since the last [`stats_zero`].
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Default ) ]
pub struct BookStats
{
  /// Orders that came to rest on the book.
  pub rests : u64,
  /// Trades produced.
  pub fills : u64,
  /// Orders refused.
  pub rejects : u64,
  /// Resting orders cancelled.
  pub cancels : u64,
}

/// A fresh, all-zero counter set.
#[ must_use ]
pub fn stats_zero() -> BookStats
{
  BookStats::default()
}

/// Record `n` more orders joining the book.
pub fn stats_rest_add( stats : &mut BookStats, n : u64 )
{
  stats.rests += n;
}

/// Record `n` more trades produced.
pub fn stats_fill_add( stats : &mut BookStats, n : u64 )
{
  stats.fills += n;
}

/// Record `n` more orders refused.
pub fn stats_reject_add( stats : &mut BookStats, n : u64 )
{
  stats.rejects += n;
}

/// Record `n` more resting orders cancelled.
pub fn stats_cancel_add( stats : &mut BookStats, n : u64 )
{
  stats.cancels += n;
}

/// A copy of the current counts, independent of further updates to `stats`.
#[ must_use ]
pub fn stats_snapshot( stats : &BookStats ) -> BookStats
{
  *stats
}
