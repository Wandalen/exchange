//! Phase P23 — a FOK order against a book too thin to fill it completely is
//! rejected, and the book afterward is identical to the book before — no
//! partial fill leaks through. Golden: `rej=1 book=same` then `ok`.

use exact_arith::{ Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_match::cross;
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_stp::SelfMatchPolicy;
use exchange_tif::Tif;

fn order( id : u64, side : Side, quantity : i64, tif : Tif ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( id ),
    side,
    price : Price::parse( "1.00" ).unwrap(),
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
  let before = book.clone();

  let taker = order( 2, Side::Buy, 10, Tif::Fok );
  let crossing = cross( &mut book, &taker, SelfMatchPolicy::CancelBoth ).unwrap();

  let rej = u8::from( crossing.trades.is_empty() );
  let same = book == before;

  println!( "rej={rej} book={}", if same { "same" } else { "different" } );
  assert_eq!( rej, 1 );
  assert!( same );
  println!( "ok" );
}
