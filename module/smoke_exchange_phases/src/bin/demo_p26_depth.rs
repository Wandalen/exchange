//! Phase P26 — `depth_top(2)` matches the book exactly. Golden: `d=1:3,0.95:4` then `ok`.
//!
//! Adapted from the design transcript's own `d=1.00:3,0.95:4`: this family's
//! real `Decimal` formatter (`exact_kind::Decimal::fmt`) trims an exactly-zero
//! fraction to nothing, decimal point included — a whole-number price like
//! `1.00` prints as `1`, never `1.00`. `0.95` has a non-zero fraction and
//! prints unchanged. Same book, same `depth_top` result; only the literal
//! zero-padding of one price differs from the transcript's own hand-written
//! example.

use exact_arith::{ Money, Quantity, money_fmt, qty_fmt };
use exchange_book::{ Book, Resting };
use exchange_depth::depth_top;
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

fn rest( id : u64, price : &str, quantity : i64 ) -> Resting
{
  let quantity = Quantity::from_int( quantity ).unwrap();
  Resting
  {
    order : Order { id : OrderId( id ), instrument : InstrumentId( 1 ), account : AccountId( id ), side : Side::Buy, price : Money::parse( price ).unwrap(), quantity, tif : Tif::Gtc, client : None },
    remaining : quantity,
    arrival : Sequence( id ),
  }
}

fn main()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, "1.00", 3 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 2, "0.95", 4 ) ), "fresh id, must succeed" );

  let depth = depth_top( &book, InstrumentId( 1 ), 2 ).unwrap();
  let line : Vec< String > = depth.bids.iter().map( | level | format!( "{}:{}", money_fmt( level.price ), qty_fmt( level.qty ) ) ).collect();
  let line = line.join( "," );

  println!( "d={line}" );
  assert_eq!( line, "1:3,0.95:4" );
  println!( "ok" );
}
