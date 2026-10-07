//! Phase P24 — `SelfMatchPolicy::CancelResting` resolves a same-account
//! crossing pair without ever producing a self-fill. Golden: `self=0` then
//! `ok`.

use exact_arith::{ Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_match::cross;
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_stp::SelfMatchPolicy;
use exchange_tif::Tif;

fn order( id : u64, side : Side, quantity : i64 ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( 1 ), // same account on both sides
    side,
    price : Price::parse( "1.00" ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif : Tif::Gtc,
    client : None,
  }
}

fn main()
{
  let mut book = Book::new();
  let resting_order = order( 1, Side::Sell, 4 );
  assert!( book.insert( Resting { order : resting_order, remaining : resting_order.quantity, arrival : Sequence( 1 ) } ) );

  let incoming = order( 2, Side::Buy, 4 );
  let crossing = cross( &mut book, &incoming, SelfMatchPolicy::CancelResting ).unwrap();

  let self_fills = crossing.trades.iter().filter( | trade | trade.taker_account == trade.maker_account ).count();

  println!( "self={self_fills}" );
  assert_eq!( self_fills, 0 );
  println!( "ok" );
}
