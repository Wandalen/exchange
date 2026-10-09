//! Test Matrix T01 — leg summation, and the batch-level assertion built on it.
//!
//! `conserve_assert`'s own tests moved from demonstrating a magnitude
//! mismatch to demonstrating an inexact notional — see `src/lib.rs`'s
//! "Revision" section for why a magnitude mismatch is no longer constructible
//! through `Trade` data at all once both of a trade's legs go in together.

use exact_arith::{ Money, Quantity };
use exchange_conserve::{ ConserveError, conserve_assert, fill_legs_sum };
use exchange_fill::Trade;
use exchange_id::{ AccountId, OrderId };
use exchange_side::Side;
use exchange_types::TypeError;

fn trade( taker_side : Side, price : &str, quantity : i64 ) -> Trade
{
  Trade
  {
    taker : OrderId( 1 ),
    taker_account : AccountId( 1 ),
    taker_side,
    maker : OrderId( 2 ),
    maker_account : AccountId( 2 ),
    price : Money::parse( price ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
  }
}

/// A trade whose notional needs more precision than the currency has — one
/// minor unit of price (`0.000001`) times one minor unit of quantity is
/// `0.000000000001`, twelve places against the currency's six. Separate from
/// `trade()` above: that helper's `quantity: i64` can only express whole
/// units, never the fractional `Quantity::EPSILON` this needs.
fn dust_trade( taker_side : Side ) -> Trade
{
  Trade
  {
    taker : OrderId( 1 ),
    taker_account : AccountId( 1 ),
    taker_side,
    maker : OrderId( 2 ),
    maker_account : AccountId( 2 ),
    price : Money::EPSILON,
    quantity : Quantity::EPSILON,
  }
}

/// T01 — a balanced leg set sums to exactly zero.
#[ test ]
fn a_balanced_leg_set_sums_to_zero()
{
  let legs = [ Money::from_int( 10 ).unwrap(), Money::from_int( -10 ).unwrap() ];
  assert_eq!( fill_legs_sum( &legs ).unwrap(), Money::ZERO );
}

/// An unbalanced leg set sums to something other than zero — `fill_legs_sum`
/// itself does not judge this, it just adds; judging is `conserve_assert`'s
/// job.
#[ test ]
fn an_unbalanced_leg_set_sums_to_its_true_imbalance()
{
  let legs = [ Money::from_int( 10 ).unwrap(), Money::from_int( -9 ).unwrap() ];
  assert_eq!( fill_legs_sum( &legs ).unwrap(), Money::from_int( 1 ).unwrap() );
}

/// A single trade conserves on its own — its own debit and credit are the
/// same number by construction.
#[ test ]
fn a_single_trade_conserves()
{
  let fills = [ trade( Side::Buy, "10", 1 ) ];
  assert_eq!( conserve_assert( &fills ), Ok( () ) );
}

/// A batch whose trades carry *different* magnitudes and a mix of
/// `taker_side` still conserves — each trade contributes both its own debit
/// and credit, so there is no way for two trades to "not offset" each other;
/// each already offsets itself. This is the behaviour the old, one-leg-per-trade
/// design could not produce (see `src/lib.rs`'s "Revision" section).
#[ test ]
fn a_mixed_batch_of_different_magnitudes_still_conserves()
{
  let fills = [ trade( Side::Buy, "10", 1 ), trade( Side::Sell, "9", 1 ), trade( Side::Buy, "2.50", 4 ) ];
  assert_eq!( conserve_assert( &fills ), Ok( () ) );
}

/// An empty batch has nothing to disagree about.
#[ test ]
fn an_empty_batch_conserves_trivially()
{
  assert_eq!( conserve_assert( &[] ), Ok( () ) );
}

/// A trade whose own notional cannot be expressed exactly is refused, not
/// silently approximated — the one failure `conserve_assert` can still
/// actually produce today (see `src/lib.rs`'s "What this still catches").
#[ test ]
fn a_trade_with_an_inexact_notional_is_refused()
{
  let fills = [ dust_trade( Side::Buy ) ];
  assert_eq!( conserve_assert( &fills ), Err( ConserveError::Notional( TypeError::NotionalInexact ) ) );
}

/// Phase P17 — `conserve_assert` passes on an ordinary batch and refuses one
/// carrying an inexact notional.
#[ test ]
fn p17_conserve_assert_distinguishes_ok_from_inexact()
{
  let ordinary = [ trade( Side::Buy, "10", 1 ), trade( Side::Sell, "10", 1 ) ];
  let inexact = [ dust_trade( Side::Buy ) ];

  assert_eq!( conserve_assert( &ordinary ), Ok( () ) );
  assert_eq!( conserve_assert( &inexact ), Err( ConserveError::Notional( TypeError::NotionalInexact ) ) );
}
