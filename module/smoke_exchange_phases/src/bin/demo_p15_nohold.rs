//! Phase P15 — a failed hold never reaches the book. Golden: `n=0` then `ok`.

use exact_arith::{ Money, Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_escrow::Escrow;
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

fn main()
{
  let buyer = AccountId( 1 );
  let mut escrow = Escrow::new();
  escrow.open( buyer, Money::ZERO, Quantity::ZERO ).unwrap(); // no funds at all

  let order = Order
  {
    id : OrderId( 1 ),
    instrument : InstrumentId( 1 ),
    account : buyer,
    side : Side::Buy,
    price : Price::parse( "2.50" ).unwrap(),
    quantity : Quantity::from_int( 4 ).unwrap(),
    tif : Tif::Gtc,
    client : None,
  };

  let mut book = Book::new();

  // The hold is attempted first; the book is only ever touched on success —
  // this `if` is the entire contract P15 exists to prove.
  if escrow.reserve( &order ).is_ok()
  {
    let remaining = order.quantity;
    assert!( book.insert( Resting { order, remaining, arrival : Sequence( 1 ) } ) );
  }

  let n = book.len();
  println!( "n={n}" );
  assert_eq!( n, 0 );
  println!( "ok" );
}
