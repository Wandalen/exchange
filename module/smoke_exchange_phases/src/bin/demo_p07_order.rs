//! Phase P07 — an order claims a sequence on arrival; a zero-quantity order is refused at submission. Golden: `seq=1 zq=1` then `ok`.

use exact_arith::{ Money, Quantity };
use exchange_core::{ AccountId, Exchange, Side };
use exchange_id::{ InstrumentId, OrderId };
use exchange_order::Order;
use exchange_seq::{ Sequence, seq_next };
use exchange_tif::Tif;

fn main()
{
  let order = Order
  {
    id : OrderId( 1 ), instrument : InstrumentId( 1 ), account : AccountId( 1 ),
    side : Side::Buy, price : Money::parse( "1" ).unwrap(), quantity : Quantity::from_int( 1 ).unwrap(),
    tif : Tif::Gtc,
  };
  let arrival = seq_next( Sequence::ZERO );

  let mut exchange = Exchange::new();
  exchange.open_account( order.account, Money::parse( "100" ).unwrap(), Quantity::from_int( 10 ).unwrap() ).unwrap();
  let zq = exchange.submit( order.account, order.side, order.price, Quantity::ZERO ).is_err();

  println!( "seq={} zq={}", arrival.0, u8::from( zq ) );
  assert_eq!( arrival.0, 1 );
  assert!( zq );
  println!( "ok" );
}
