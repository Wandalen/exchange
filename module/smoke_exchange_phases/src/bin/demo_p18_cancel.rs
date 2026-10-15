//! Phase P18 — cancel actually removes the resting order, not merely makes
//! it unreachable by id. Golden: `n=0` then `ok`.

use exact_arith::{ Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_rest::{ rest_cancel, rest_place };
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

fn main()
{
  let order = Order
  {
    id : OrderId( 1 ),
    instrument : InstrumentId( 1 ),
    account : AccountId( 1 ),
    side : Side::Buy,
    price : Price::parse( "1.00" ).unwrap(),
    quantity : Quantity::from_int( 4 ).unwrap(),
    tif : Tif::Gtc,
    client : None,
  };

  let mut book = Book::new();
  assert!( rest_place( &mut book, Resting { order, remaining : order.quantity, arrival : Sequence( 1 ) } ) );

  assert!( rest_cancel( &mut book, InstrumentId( 1 ), OrderId( 1 ) ).is_some() );

  let n = book.len();
  println!( "n={n}" );
  assert_eq!( n, 0 );
  println!( "ok" );
}
