//! Phase P20 — a taker that outsizes one maker fills against a second and
//! leaves the right remainder. Golden: `fills=10,2 rest=3` then `ok`.

use exact_arith::{ Money, Quantity };
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
    account : AccountId( id ),
    side,
    price : Money::parse( "1.00" ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif : Tif::Gtc,
    client : None,
  }
}

fn main()
{
  let mut book = Book::new();
  let maker1 = order( 1, Side::Buy, 10 );
  let maker2 = order( 2, Side::Buy, 5 );
  assert!( book.insert( Resting { order : maker1, remaining : maker1.quantity, arrival : Sequence( 1 ) } ) );
  assert!( book.insert( Resting { order : maker2, remaining : maker2.quantity, arrival : Sequence( 2 ) } ) );

  let taker = order( 3, Side::Sell, 12 );
  let crossing = cross( &mut book, &taker, SelfMatchPolicy::CancelBoth ).unwrap();

  assert_eq!( crossing.trades.len(), 2, "one trade per maker swept" );
  let rest = book.best( InstrumentId( 1 ), Side::Buy ).unwrap().remaining;

  println!( "fills={},{} rest={}", crossing.trades[ 0 ].quantity, crossing.trades[ 1 ].quantity, rest );
  assert_eq!( crossing.trades[ 0 ].quantity, Quantity::from_int( 10 ).unwrap() );
  assert_eq!( crossing.trades[ 1 ].quantity, Quantity::from_int( 2 ).unwrap() );
  assert_eq!( rest, Quantity::from_int( 3 ).unwrap() );
  println!( "ok" );
}
