//! Phase P16 — one cross produces a `Trade` naming both the maker and the
//! taker. Golden: `maker=a taker=b` then `ok`.

use exact_arith::{ Money, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_match::cross;
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_stp::SelfMatchPolicy;
use exchange_tif::Tif;

fn order( id : u64, account : u64, side : Side, quantity : i64 ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( account ),
    side,
    price : Money::parse( "2.50" ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif : Tif::Gtc,
  }
}

fn main()
{
  let maker_order = order( 1, 1, Side::Sell, 4 ); // "a"
  let taker_order = order( 2, 2, Side::Buy, 4 ); // "b"

  let mut book = Book::new();
  assert!( book.insert( Resting { order : maker_order, remaining : maker_order.quantity, arrival : Sequence( 1 ) } ) );

  let crossing = cross( &mut book, &taker_order, SelfMatchPolicy::CancelBoth ).unwrap();
  let trade = crossing.trades.first().unwrap();

  let maker = if trade.maker == maker_order.id { "a" } else { "b" };
  let taker = if trade.taker == taker_order.id { "b" } else { "a" };

  println!( "maker={maker} taker={taker}" );
  assert_eq!( maker, "a" );
  assert_eq!( taker, "b" );
  println!( "ok" );
}
