//! Test Matrix T01 — monotonicity, and Phase P08's own smoke assertion.

use exchange_seq::{ Sequence, seq_next };

/// T01 — `seq_next` is strictly increasing.
#[ test ]
fn seq_next_is_strictly_increasing()
{
  let first = seq_next( Sequence::ZERO );
  let second = seq_next( first );

  assert!( second > first );
  assert!( first > Sequence::ZERO );
}

/// T01 — `ZERO` is the starting value, never produced by `seq_next`.
#[ test ]
fn zero_is_the_starting_value()
{
  assert_eq!( Sequence::ZERO, Sequence( 0 ) );
  assert_ne!( seq_next( Sequence::ZERO ), Sequence::ZERO );
}

/// Phase P08 — three successive `seq_next` calls are monotonic.
///
/// Golden print: `s=1,2,3` then `ok` (`docs/golden_output/008_p08_golden.md`).
#[ test ]
fn p08_seq_next_monotonic()
{
  let a = seq_next( Sequence::ZERO );
  let b = seq_next( a );
  let c = seq_next( b );

  assert_eq!( ( a.0, b.0, c.0 ), ( 1, 2, 3 ) );
}
