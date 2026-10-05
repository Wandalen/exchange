//! The wall scenario's independent pieces, driven without the binary.
//!
//! [`run`] checks everything it checks by panicking, and the last test here
//! covers that end to end — but each of the four independently-measurable
//! facts below also gets its own test. A regression that only breaks one of
//! them should name that one, not force a reader into the whole golden
//! block to find out which.
//!
//! [`run`]: smoke_exchange_book::run

use smoke_exchange_book::
{
  conserve_ok, duplicate_rejected, ring_determinism, ring_overflow_rejects, run, scene, self_trade_fill_count, units,
};

/// The GTC taker takes two levels at the same price, in arrival order, and
/// leaves the rest for the depth read to find.
#[ test ]
fn the_scene_fills_two_levels_and_leaves_the_right_remainder()
{
  let report = scene();

  assert_eq!( report.gtc_fills.len(), 2, "12 against 10-then-5 should produce two fills" );
  assert_eq!( report.gtc_fills[ 0 ].quantity, units( 10 ), "the earlier bid should be exhausted first" );
  assert_eq!( report.gtc_fills[ 1 ].quantity, units( 2 ), "the second fill should only take what the order still wanted" );
  assert_eq!( report.rest_bid_qty, units( 3 ), "5 - 2 = 3 should remain of the second bid" );
}

/// The IOC taker sweeps what it can and drops the rest — it never rests a
/// remainder, which is the bug this scenario was written to catch.
#[ test ]
fn the_ioc_taker_drops_its_remainder_instead_of_resting_it()
{
  let report = scene();

  assert_eq!( report.ioc_fills.len(), 2, "the IOC should sweep both remaining levels" );
  assert_eq!( report.ioc_rest, units( 0 ), "an IOC must never rest its unfilled remainder" );
}

/// The FOK taker against an empty book is rejected whole, and the halt
/// round trip blocks then allows the same placement.
#[ test ]
fn the_fok_taker_rejects_and_the_halt_round_trip_both_behave()
{
  let report = scene();

  assert!( report.fok_rejected, "a FOK against a thin book must be rejected whole" );
  assert!( report.halt_then_resume_ok, "halting must block a placement and clearing must allow it again" );
}

/// The self-trade probe produces no fill — an account never trades against
/// its own resting order. Built on its own fresh exchange, so this holds
/// regardless of what the main scene already did.
#[ test ]
fn the_self_trade_probe_produces_no_fill()
{
  assert_eq!( self_trade_fill_count(), 0, "an account must never trade against its own resting order" );
}

/// A duplicate `OrderId` is refused by `exchange_idem` on its second
/// sighting, standalone — this property has no path through the facade at
/// all, so it is only ever checked here.
#[ test ]
fn a_duplicate_order_id_is_refused_standalone()
{
  assert!( duplicate_rejected(), "the second sighting of the same id must be refused" );
}

/// Two independent two-ring races reproduce the same checksum, despite
/// being genuinely raced across real OS threads.
#[ test ]
fn two_ring_races_reproduce_the_same_checksum()
{
  let ( a, b ) = ring_determinism();
  assert_eq!( a, b, "the same two producers, drained in the same fixed order, must reproduce the same checksum" );
}

/// A ring at capacity 2 refuses a third publish as an explicit reject,
/// never a silent drop.
#[ test ]
fn a_full_ring_rejects_rather_than_drops()
{
  assert!( ring_overflow_rejects(), "a full ring must reject the third publish, not drop it" );
}

/// The scene's own real fills conserve — checked here independently of
/// `scene`'s internal assertions, since nothing in the matching path checks
/// this on its own behalf.
#[ test ]
fn the_scenes_real_fills_conserve()
{
  let report = scene();
  let all_fills : Vec< _ > = report.gtc_fills.iter().copied().chain( report.ioc_fills.iter().copied() ).collect();
  assert!( conserve_ok( &all_fills ), "a batch of real fills must always conserve" );
}

/// The lane runs end to end and every field matches the golden block.
///
/// [`run`] checks everything it checks by panicking, so reaching the end of
/// this test is the verdict — the same verdict the binary reports, taken
/// here where the suite can see it.
#[ test ]
fn the_lane_runs_end_to_end()
{
  run();
}
