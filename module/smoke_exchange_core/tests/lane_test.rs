//! The lane's three arms, driven without the binary.
//!
//! The lane is its own test — every claim it prints is an assertion — but that
//! only holds while something runs it, and until the lane was split out of
//! `src/main.rs` the only thing that did was a person typing `cargo run`.
//! No test suite can reach a bare `src/main.rs` to verify it automatically, so
//! the file carrying every assertion about the exchange was also the file
//! nothing measured.
//!
//! The control arm gets a test of its own rather than only running inside
//! [`run`]. Its whole purpose is to report zero, and a zero that stops being
//! zero makes the crossing arm meaningless rather than making it fail — so the
//! disagreement is worth asserting where a failure can name it.
//!
//! [`run`]: smoke_exchange_core::run

use smoke_exchange_core::{ OPENING_CASH, cancel_arm, control_arm, crossing_arm, money, run };

/// The crossing arm fills once, and the cash moved rather than appearing.
///
/// Both sides are asserted, not just the trade count: an engine that credited
/// the seller without debiting the buyer would fill exactly once and report a
/// perfectly plausible trade.
#[ test ]
fn the_crossing_arm_fills_once_and_moves_the_cash()
{
  let ( filled, seller_cash, buyer_cash ) = crossing_arm();

  assert_eq!( filled, 1, "the bid did not take the one resting ask" );
  assert_eq!( seller_cash, money( "1010" ), "the seller was not paid the traded value" );
  assert_eq!( buyer_cash, money( "990" ), "the buyer did not pay the traded value" );
}

/// The control arm declines, and the two arms disagree.
///
/// The disagreement is the point. An engine that filled unconditionally would
/// satisfy every assertion in the crossing arm, so the crossing arm says
/// nothing on its own about whether the engine can decline.
#[ test ]
fn the_control_arm_declines_and_disagrees_with_the_crossing_arm()
{
  let declined = control_arm();
  assert_eq!( declined, 0, "a bid one minor unit below the ask crossed it" );

  let ( filled, .. ) = crossing_arm();
  assert_ne!( filled, declined, "the two arms agreed, so neither has shown anything" );
}

/// A cancelled order returns every committed unit.
///
/// Asserted against the opening balance rather than against a difference: a
/// reservation returned to the wrong place would still change the balance, and
/// only the original figure says it went back where it came from.
#[ test ]
fn a_cancel_returns_the_whole_reservation()
{
  assert_eq!
  (
    cancel_arm(),
    money( OPENING_CASH ),
    "the cancel did not return every committed unit to the account it came from"
  );
}

/// The lane runs end to end and every arm's assertion holds.
///
/// [`run`] checks everything it checks by panicking, so reaching the end of
/// this test is the verdict — the same verdict the binary reports, taken here
/// where the suite can see it.
#[ test ]
fn the_lane_runs_end_to_end()
{
  run();
}
