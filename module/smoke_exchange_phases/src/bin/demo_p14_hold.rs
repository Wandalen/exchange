//! Phase P14 — a hold moves through its full lifecycle: release on cancel,
//! commit on fill. Golden: `rel=1 com=1` then `ok`.

use exact_arith::{ Money, Quantity };
use exchange_escrow::Escrow;
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_side::Side;
use exchange_tif::Tif;
use exchange_types::Trade;

fn order( id : u64, account : u64, side : Side, price : &str, quantity : i64 ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( account ),
    side,
    price : Money::parse( price ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif : Tif::Gtc,
  }
}

fn main()
{
  let ( seller, buyer ) = ( AccountId( 1 ), AccountId( 2 ) );
  let mut escrow = Escrow::new();
  escrow.open( seller, Money::ZERO, Quantity::from_int( 10 ).unwrap() ).unwrap();
  escrow.open( buyer, Money::from_int( 1000 ).unwrap(), Quantity::ZERO ).unwrap();

  // Hold, then release on cancel — the first transition.
  let cancelled = order( 1, 1, Side::Sell, "2.50", 1 );
  escrow.reserve( &cancelled ).unwrap();
  let rel = escrow.release( seller, cancelled.id ).is_ok();

  // Hold, then commit on fill — the second transition.
  let sell = order( 2, 1, Side::Sell, "2.50", 4 );
  let buy = order( 3, 2, Side::Buy, "2.50", 4 );
  escrow.reserve( &sell ).unwrap();
  escrow.reserve( &buy ).unwrap();

  let trade = Trade
  {
    taker : buy.id,
    taker_account : buy.account,
    taker_side : Side::Buy,
    maker : sell.id,
    maker_account : sell.account,
    price : buy.price,
    quantity : buy.quantity,
  };
  let com = escrow.settle( &trade, Side::Buy, buy.price ).is_ok();

  println!( "rel={} com={}", u8::from( rel ), u8::from( com ) );
  assert!( rel );
  assert!( com );
  println!( "ok" );
}
