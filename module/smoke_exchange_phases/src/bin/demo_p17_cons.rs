//! Phase P17 — an ordinary batch conserves to zero; a trade whose own
//! notional is inexact is correctly refused. Golden: `z=1 nz=1` then `ok`.
//!
//! Originally built around a 10/-10 vs 10/-9 leg mismatch, matching the
//! family's proposal literally — revised once wiring `conserve_assert`
//! against a real `exchange_match::cross` batch proved that shape
//! unreachable: both of a trade's legs go in together, so two trades can
//! never fail to offset each other. See `exchange_conserve/src/lib.rs`'s
//! "Revision" section.

use exact_arith::{ Money, Quantity };
use exchange_conserve::conserve_assert;
use exchange_id::{ AccountId, OrderId };
use exchange_side::Side;
use exchange_types::{ Price, Trade };

fn trade( taker_side : Side, price : &str ) -> Trade
{
  Trade
  {
    taker : OrderId( 1 ),
    taker_account : AccountId( 1 ),
    taker_side,
    maker : OrderId( 2 ),
    maker_account : AccountId( 2 ),
    price : Price::parse( price ).unwrap(),
    quantity : Quantity::from_int( 1 ).unwrap(),
  }
}

fn main()
{
  let ordinary = [ trade( Side::Sell, "10" ), trade( Side::Buy, "9" ) ];
  let z = conserve_assert( &ordinary ).is_ok();

  let inexact = Trade
  {
    taker : OrderId( 1 ),
    taker_account : AccountId( 1 ),
    taker_side : Side::Buy,
    maker : OrderId( 2 ),
    maker_account : AccountId( 2 ),
    price : Money::EPSILON,
    quantity : Quantity::EPSILON,
  };
  let nz = conserve_assert( &[ inexact ] ).is_err();

  println!( "z={} nz={}", u8::from( z ), u8::from( nz ) );
  assert!( z );
  assert!( nz );
  println!( "ok" );
}
