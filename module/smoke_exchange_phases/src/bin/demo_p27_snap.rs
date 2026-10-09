//! Phase P27 — a snapshot taken before a live cancel still shows the
//! cancelled row afterward. Golden: `snap=1 live=0` then `ok`.

use exact_arith::{ Money, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_snap::{ snap_len, snap_take };
use exchange_tif::Tif;

fn main()
{
  let mut book = Book::new();
  let quantity = Quantity::from_int( 3 ).unwrap();
  assert!( book.insert( Resting
  {
    order : Order { id : OrderId( 1 ), instrument : InstrumentId( 1 ), account : AccountId( 1 ), side : Side::Buy, price : Money::parse( "1.00" ).unwrap(), quantity, tif : Tif::Gtc, client : None },
    remaining : quantity,
    arrival : Sequence( 1 ),
  } ), "fresh id, must succeed" );

  let snap = snap_take( &book, InstrumentId( 1 ), Money::parse( "0.01" ).unwrap() );
  assert!( book.cancel( InstrumentId( 1 ), OrderId( 1 ) ).is_some(), "the order must actually be there to cancel" );

  println!( "snap={} live={}", snap_len( &snap ), book.len() );
  assert_eq!( snap_len( &snap ), 1 );
  assert_eq!( book.len(), 0 );
  println!( "ok" );
}
