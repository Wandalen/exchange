//! Running counters for the hot path: how many rests, fills, and rejects a
//! book has seen.
//!
//! A root of the dependency tree: no dependency on any other `exchange_*`
//! crate. Without this crate, the only record of what the match loop did is
//! the full event log — answering "how many rejects so far" means re-scanning
//! every event rather than reading a number that was already being kept.
//!
//! The source design names `exchange_id` as a dependency; nothing here
//! actually needs an id type — every counter is a plain running total, not
//! keyed by any identity — so the dependency is not taken. Recorded as this
//! crate's own [`docs/decisions`](docs/decisions) entry rather than matched
//! silently.
//!
//! # Genuinely new: `stats_rest_add`, `cancels`, `stats_cancel_add`
//!
//! The source design's own `BookStats` shape is
//! `{ rests, fills, rejects, cancels }` (`core_exchange.txt:648`), but its
//! exposed-item *function* list names only
//! `stats_zero`/`stats_fill_add`/`stats_reject_add`/`stats_snapshot` —
//! leaving both `rests` and `cancels` with no function that increments
//! either. A field nothing can ever set past zero is not a counter,
//! so `stats_rest_add` and `stats_cancel_add` are both added for symmetry
//! with `stats_fill_add`/`stats_reject_add`, not left as gaps the proposal's
//! own function list happened to miss.

/// Running counts for one book: how many rests, fills, and rejects it has
/// seen since the last [`stats_zero`].
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Default ) ]
pub struct BookStats
{
  /// Orders that joined the book and are still (or were) resting.
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
