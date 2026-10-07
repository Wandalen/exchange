//! Phase P21 — a worse price level is never touched while a better one
//! still has quantity to give. Golden: `untouched=1` then `ok`.

use exact_arith::{ Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_match::cross;
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_stp::SelfMatchPolicy;
use exchange_tif::Tif;

fn order( id : u64, side : Side, price : &str, quantity : i64 ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( id ),
    side,
    price : Price::parse( price ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif : Tif::Gtc,
    client : None,
  }
}

fn main()
{
  let mut book = Book::new();
  let better = order( 1, Side::Buy, "1.00", 4 );
  let worse = order( 2, Side::Buy, "0.95", 4 );
  assert!( book.insert( Resting { order : better, remaining : better.quantity, arrival : Sequence( 1 ) } ) );
  assert!( book.insert( Resting { order : worse, remaining : worse.quantity, arrival : Sequence( 2 ) } ) );

  // Willing to sell as low as 0.90 — reaches both levels on price, but the
  // better one alone has enough quantity, so the worse level must stay
  // untouched purely on priority, not because it was unreachable.
  let taker = order( 3, Side::Sell, "0.90", 2 );
  let crossing = cross( &mut book, &taker, SelfMatchPolicy::CancelBoth ).unwrap();
  assert_eq!( crossing.trades.len(), 1, "the better level alone had enough" );

  let worse_after = book.iter().find( | resting | resting.order.id == OrderId( 2 ) ).unwrap();
  let untouched = u8::from( worse_after.remaining == worse.quantity );

  println!( "untouched={untouched}" );
  assert_eq!( untouched, 1 );
  println!( "ok" );
}
