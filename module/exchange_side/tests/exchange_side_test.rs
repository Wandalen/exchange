//! Test Matrix T01 — the opposite relation, and Phase P02's own smoke assertion.

use exchange_side::{ Side, side_accepts, side_ahead, side_is_ask, side_is_bid, side_opposite };

/// T01 — every side has exactly one opposite.
#[ test ]
fn every_side_has_exactly_one_opposite()
{
  assert_eq!( Side::Buy.opposite(), Side::Sell );
  assert_eq!( Side::Sell.opposite(), Side::Buy );
  assert_eq!( Side::Buy.opposite().opposite(), Side::Buy );
}

/// The free-function form agrees with the method.
#[ test ]
fn side_opposite_agrees_with_the_method()
{
  assert_eq!( side_opposite( Side::Buy ), Side::Buy.opposite() );
}

/// The bid/ask query helpers are each other's exact complement.
#[ test ]
fn bid_and_ask_are_complementary()
{
  assert!( side_is_bid( Side::Buy ) );
  assert!( !side_is_ask( Side::Buy ) );
  assert!( side_is_ask( Side::Sell ) );
  assert!( !side_is_bid( Side::Sell ) );
}

/// A higher bid and a lower ask rank ahead; an equal price does not.
#[ test ]
fn side_ahead_ranks_by_aggressiveness()
{
  assert!( side_ahead( Side::Buy, 11, 10 ) );
  assert!( !side_ahead( Side::Buy, 10, 11 ) );
  assert!( side_ahead( Side::Sell, 10, 11 ) );
  assert!( !side_ahead( Side::Sell, 11, 10 ) );
  assert!( !side_ahead( Side::Buy, 10, 10 ) );
  assert!( !side_ahead( Side::Sell, 10, 10 ) );
}

/// A buy accepts at or below its limit, a sell at or above.
#[ test ]
fn side_accepts_is_inclusive_at_the_limit()
{
  assert!( side_accepts( Side::Buy, 10, 9 ) );
  assert!( side_accepts( Side::Buy, 10, 10 ) );
  assert!( !side_accepts( Side::Buy, 10, 11 ) );
  assert!( side_accepts( Side::Sell, 10, 11 ) );
  assert!( side_accepts( Side::Sell, 10, 10 ) );
  assert!( !side_accepts( Side::Sell, 10, 9 ) );
}

/// Phase P02 — the opposite of bid is ask.
///
/// Golden print: `opp=ask` then `ok` (`docs/golden_output/002_p02_golden.md`).
#[ test ]
fn p02_side_opposite()
{
  let opposite = Side::Buy.opposite();
  assert!( side_is_ask( opposite ) );
}
