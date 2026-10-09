//! Phase P11 — walking a side visits prices in sorted order, best first,
//! never a hash/map iteration order. Golden: `w=1,0.95` then `ok`.
//!
//! Adapted from the design transcript's own `w=1.00,0.95`: this family's
//! real `Decimal` formatter trims an exactly-zero fraction to nothing,
//! decimal point included — see `demo_p26_depth`'s own module doc for the
//! same adaptation on the same formatter. `0.95` has a non-zero fraction and
//! prints unchanged.

use exact_arith::{ Money, Quantity, money_fmt };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

fn rest( id : u64, price : &str ) -> Resting
{
  let quantity = Quantity::from_int( 1 ).unwrap();
  Resting
  {
    order : Order
    {
      id : OrderId( id ), instrument : InstrumentId( 1 ), account : AccountId( id ),
      side : Side::Buy, price : Money::parse( price ).unwrap(), quantity, tif : Tif::Gtc,
      client : None,
    },
    remaining : quantity,
    arrival : Sequence( id ),
  }
}

fn main()
{
  let instrument = InstrumentId( 1 );
  let mut book = Book::new();

  // Inserted worse-first — a walk that merely followed insertion order, or a
  // hash iteration order, would be caught by this.
  assert!( book.insert( rest( 1, "0.95" ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 2, "1.00" ) ), "fresh id, must succeed" );

  let prices : Vec< String > = book.side( instrument, Side::Buy )
  .map( | resting | money_fmt( resting.order.price ) )
  .collect();
  let line = prices.join( "," );

  println!( "w={line}" );
  assert_eq!( line, "1,0.95" );
  println!( "ok" );
}
