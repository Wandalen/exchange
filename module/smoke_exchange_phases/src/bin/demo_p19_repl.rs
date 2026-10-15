//! Phase P19 — replace is atomic: the old order is gone exactly when the new
//! one stands, never both, never neither. Golden: `old=0 new=1` then `ok`.

use exact_arith::{ Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_rest::{ rest_place, rest_replace };
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

fn order( id : u64, price : &str, quantity : i64 ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( 1 ),
    side : Side::Buy,
    price : Price::parse( price ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif : Tif::Gtc,
    client : None,
  }
}

fn main()
{
  let mut book = Book::new();
  let old_order = order( 1, "1.00", 4 );
  assert!( rest_place( &mut book, Resting { order : old_order, remaining : old_order.quantity, arrival : Sequence( 1 ) } ) );

  let new_order = order( 2, "1.01", 6 );
  let new_resting = Resting { order : new_order, remaining : new_order.quantity, arrival : Sequence( 2 ) };
  assert!( rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), new_resting ).is_ok() );

  let old = u8::from( book.iter().any( | r | r.order.id == OrderId( 1 ) ) );
  let new = u8::from( book.iter().any( | r | r.order.id == OrderId( 2 ) ) );

  println!( "old={old} new={new}" );
  assert_eq!( old, 0 );
  assert_eq!( new, 1 );
  println!( "ok" );
}
