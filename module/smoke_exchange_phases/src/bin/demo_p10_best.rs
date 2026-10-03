//! Phase P10 — the book tracks best bid and best ask after inserts, without
//! a full scan. Golden: `best=1` then `ok`.
//!
//! Adapted from the design transcript's own `best=1.00`: this family's real
//! `Decimal` formatter trims an exactly-zero fraction to nothing, decimal
//! point included — see `demo_p26_depth`'s own module doc for the same
//! adaptation on the same formatter.

use exact_arith::{ Money, Quantity, money_fmt };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_side::Side;
use exchange_tif::Tif;
use exchange_types::{ Order, Sequence };

fn rest( id : u64, side : Side, price : &str ) -> Resting
{
  let quantity = Quantity::from_int( 1 ).unwrap();
  Resting
  {
    order : Order
    {
      id : OrderId( id ), instrument : InstrumentId( 1 ), account : AccountId( id ),
      side, price : Money::parse( price ).unwrap(), quantity, tif : Tif::Gtc,
    },
    remaining : quantity,
    arrival : Sequence( id ),
  }
}

fn main()
{
  let instrument = InstrumentId( 1 );
  let mut book = Book::new();

  assert!( book.insert( rest( 1, Side::Sell, "2.00" ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 2, Side::Buy, "1.00" ) ), "fresh id, must succeed" );

  let best_bid = book.best( instrument, Side::Buy ).unwrap().order.price;
  let best_ask = book.best( instrument, Side::Sell ).unwrap().order.price;

  println!( "best={}", money_fmt( best_bid ) );
  assert_eq!( best_bid, Money::parse( "1.00" ).unwrap() );
  assert_eq!( best_ask, Money::parse( "2.00" ).unwrap() );
  println!( "ok" );
}
