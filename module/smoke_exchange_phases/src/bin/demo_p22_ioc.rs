//! Phase P22 — an IOC order never rests its remainder. `cross` itself never
//! inserts a remainder for any TIF (see `exchange_match`'s module doc); the
//! caller orchestration shown here consults `tif_rests` before calling
//! `rest_place`, and skips it for IOC. Golden: `rest=0` then `ok`.

use exact_arith::{ Money, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_match::cross;
use exchange_order::Order;
use exchange_rest::rest_place;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_stp::SelfMatchPolicy;
use exchange_tif::{ Tif, tif_rests };

fn order( id : u64, side : Side, quantity : i64, tif : Tif ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( id ),
    side,
    price : Money::parse( "1.00" ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif,
    client : None,
  }
}

fn main()
{
  let mut book = Book::new();
  let maker = order( 1, Side::Sell, 4, Tif::Gtc );
  assert!( book.insert( Resting { order : maker, remaining : maker.quantity, arrival : Sequence( 1 ) } ) );

  let taker = order( 2, Side::Buy, 10, Tif::Ioc );
  let crossing = cross( &mut book, &taker, SelfMatchPolicy::CancelBoth ).unwrap();
  assert!( crossing.remaining > Quantity::ZERO, "the taker outsized the book, leaving a remainder to decide about" );

  if tif_rests( taker.tif )
  {
    let remainder = Resting { order : taker, remaining : crossing.remaining, arrival : Sequence( 2 ) };
    assert!( rest_place( &mut book, remainder ) );
  }

  let rest = book.iter().filter( | r | r.order.id == taker.id ).count();
  println!( "rest={rest}" );
  assert_eq!( rest, 0 );
  println!( "ok" );
}
