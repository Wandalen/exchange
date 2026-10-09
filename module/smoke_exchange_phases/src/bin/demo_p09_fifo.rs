//! Phase P09 — two orders rest at the same price; the earlier arrival pops first. Golden: `first=a` then `ok`.

use exact_arith::{ Money, Quantity };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_level::{ LevelNode, level_new, level_pop_front, level_push };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

fn node( id : u64, arrival : u64 ) -> LevelNode
{
  let quantity = Quantity::from_int( 1 ).unwrap();
  LevelNode
  {
    order : Order
    {
      id : OrderId( id ), instrument : InstrumentId( 1 ), account : AccountId( id ),
      side : Side::Buy, price : Money::parse( "1" ).unwrap(), quantity, tif : Tif::Gtc,
      client : None,
    },
    remaining : quantity,
    arrival : Sequence( arrival ),
  }
}

fn main()
{
  let mut level = level_new( Money::parse( "1" ).unwrap() );
  level_push( &mut level, node( 1, 1 ) ); // "a" — earlier arrival
  level_push( &mut level, node( 2, 2 ) ); // "b" — later arrival

  let first = level_pop_front( &mut level ).unwrap();
  let label = if first.order.id == OrderId( 1 ) { "a" } else { "b" };

  println!( "first={label}" );
  assert_eq!( label, "a" );
  println!( "ok" );
}
