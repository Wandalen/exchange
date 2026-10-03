//! Test Matrix T01 — counter accumulation and snapshot independence.

use exchange_stats::{ BookStats, stats_cancel_add, stats_fill_add, stats_reject_add, stats_rest_add, stats_snapshot, stats_zero };

/// T01 — a fresh counter set starts at all zero.
#[ test ]
fn zero_is_all_zero()
{
  assert_eq!( stats_zero(), BookStats { rests : 0, fills : 0, rejects : 0, cancels : 0 } );
}

/// T01 — each counter accumulates independently of the other three.
#[ test ]
fn adds_accumulate_independently()
{
  let mut stats = stats_zero();
  stats_rest_add( &mut stats, 3 );
  stats_fill_add( &mut stats, 2 );
  stats_reject_add( &mut stats, 1 );
  stats_cancel_add( &mut stats, 2 );
  stats_rest_add( &mut stats, 1 );

  assert_eq!( stats, BookStats { rests : 4, fills : 2, rejects : 1, cancels : 2 } );
}

/// T01 — a snapshot is a copy, not a view: later updates to the live
/// counter never retroactively change an already-taken snapshot.
#[ test ]
fn snapshot_does_not_alias_the_live_counter()
{
  let mut stats = stats_zero();
  stats_fill_add( &mut stats, 5 );

  let snap = stats_snapshot( &stats );
  stats_fill_add( &mut stats, 5 );

  assert_eq!( snap.fills, 5 );
  assert_eq!( stats.fills, 10 );
}
