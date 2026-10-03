//! Test Matrix T01 — the cap check, and Phase P12's own smoke assertion.

use exchange_cap::{ BookCaps, CapError, cap_check_level, cap_check_rest };

const CAPS : BookCaps = BookCaps { max_rests : 2, max_levels : 2 };

/// T01 — a rest below the cap is accepted.
#[ test ]
fn a_rest_below_the_cap_is_accepted()
{
  assert_eq!( cap_check_rest( CAPS, 0 ), Ok( () ) );
  assert_eq!( cap_check_rest( CAPS, 1 ), Ok( () ) );
}

/// T01 — a rest at the cap is refused by name, not silently dropped.
#[ test ]
fn a_rest_at_the_cap_is_refused()
{
  assert_eq!( cap_check_rest( CAPS, 2 ), Err( CapError::RestsFull ) );
}

/// T01 — the level cap is checked and refused independently of the rest cap.
#[ test ]
fn a_level_at_the_cap_is_refused()
{
  assert_eq!( cap_check_level( CAPS, 2 ), Err( CapError::LevelsFull ) );
}

/// Phase P12 — with a rest cap of 2, a third rest is refused.
///
/// Golden print: `ok_full` directly (`docs/golden_output/012_p12_golden.md`).
#[ test ]
fn p12_third_rest_is_refused()
{
  assert_eq!( cap_check_rest( CAPS, 0 ), Ok( () ) );
  assert_eq!( cap_check_rest( CAPS, 1 ), Ok( () ) );
  assert_eq!( cap_check_rest( CAPS, 2 ), Err( CapError::RestsFull ) );
}
