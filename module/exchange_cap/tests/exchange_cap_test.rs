//! Test Matrix T01 — the cap check, and Phase P12's own smoke assertion.

use exchange_cap::{ BookCaps, CapError, cap_check_account, cap_check_level, cap_check_rest };

const CAPS : BookCaps = BookCaps { max_rests : 2, max_levels : 2, max_account_rests : 2 };

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

/// T01 — a level below the cap is accepted.
#[ test ]
fn a_level_below_the_cap_is_accepted()
{
  assert_eq!( cap_check_level( CAPS, 0 ), Ok( () ) );
  assert_eq!( cap_check_level( CAPS, 1 ), Ok( () ) );
}

/// T01 — the level cap is checked and refused independently of the rest cap.
#[ test ]
fn a_level_at_the_cap_is_refused()
{
  assert_eq!( cap_check_level( CAPS, 2 ), Err( CapError::LevelsFull ) );
}

/// T01 — the account cap accepts below its limit and refuses at it.
#[ test ]
fn an_account_at_the_cap_is_refused()
{
  assert_eq!( cap_check_account( CAPS, 1 ), Ok( () ) );
  assert_eq!( cap_check_account( CAPS, 2 ), Err( CapError::AccountFull ) );
}

/// T01 — a zero cap refuses the very first rest, level and account order.
#[ test ]
fn a_zero_cap_refuses_everything()
{
  let caps = BookCaps { max_rests : 0, max_levels : 0, max_account_rests : 0 };

  assert_eq!( cap_check_rest( caps, 0 ), Err( CapError::RestsFull ) );
  assert_eq!( cap_check_level( caps, 0 ), Err( CapError::LevelsFull ) );
  assert_eq!( cap_check_account( caps, 0 ), Err( CapError::AccountFull ) );
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
