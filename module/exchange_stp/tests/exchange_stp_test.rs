//! Test Matrix T01 — the three policies, and Phase P04's own smoke assertion.

use exchange_stp::{ SelfMatchPolicy, stp_name };

/// T01 — the three policies are mutually distinct.
#[ test ]
fn the_three_policies_are_mutually_distinct()
{
  assert_ne!( SelfMatchPolicy::CancelResting, SelfMatchPolicy::CancelIncoming );
  assert_ne!( SelfMatchPolicy::CancelIncoming, SelfMatchPolicy::CancelBoth );
  assert_ne!( SelfMatchPolicy::CancelResting, SelfMatchPolicy::CancelBoth );
}

/// Every policy has a distinct, stable name.
#[ test ]
fn every_policy_has_a_distinct_name()
{
  let names =
  [
    stp_name( SelfMatchPolicy::CancelResting ),
    stp_name( SelfMatchPolicy::CancelIncoming ),
    stp_name( SelfMatchPolicy::CancelBoth ),
  ];

  assert_eq!( names, [ "cancel_resting", "cancel_incoming", "cancel_both" ] );
}

/// Phase P04 — three self-trade policies exist, named and distinguishable.
///
/// Golden print: `n=3` then `ok` (`docs/golden_output/004_p04_golden.md`).
#[ test ]
fn p04_stp_policy_count()
{
  let policies =
  [ SelfMatchPolicy::CancelResting, SelfMatchPolicy::CancelIncoming, SelfMatchPolicy::CancelBoth ];

  let mut n = 0;
  for ( i, a ) in policies.iter().enumerate()
  {
    if policies[ ..i ].iter().all( | b | b != a )
    {
      n += 1;
    }
  }

  assert_eq!( n, 3 );
}
