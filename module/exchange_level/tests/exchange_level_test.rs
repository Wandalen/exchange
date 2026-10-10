//! `Level`'s whole contract: one price, FIFO arrival order, nothing else.

use exact_arith::{ Price, Quantity };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_level::
{
  LevelNode, level_empty_is, level_len, level_new, level_pop_front, level_push,
  level_qty_sum, level_remove,
};
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

fn node( id : u64, quantity : i64, arrival : u64 ) -> LevelNode
{
  LevelNode
  {
    order : Order
    {
      id : OrderId( id ),
      instrument : InstrumentId( 1 ),
      account : AccountId( id ),
      side : Side::Buy,
      price : Price::parse( "2.50" ).unwrap(),
      quantity : Quantity::from_int( quantity ).unwrap(),
      tif : Tif::Gtc,
      client : None,
    },
    remaining : Quantity::from_int( quantity ).unwrap(),
    arrival : Sequence( arrival ),
  }
}

#[ test ]
fn a_new_level_holds_its_price_and_nothing_else()
{
  let price = Price::parse( "2.50" ).unwrap();
  let level = level_new( price );

  assert_eq!( level.price, price );
  assert!( level_empty_is( &level ) );
  assert_eq!( level_len( &level ), 0 );
}

#[ test ]
fn pushed_nodes_pop_in_the_order_they_arrived()
{
  let mut level = level_new( Price::parse( "2.50" ).unwrap() );
  level_push( &mut level, node( 1, 4, 1 ) );
  level_push( &mut level, node( 2, 4, 2 ) );
  level_push( &mut level, node( 3, 4, 3 ) );

  assert_eq!( level_pop_front( &mut level ).unwrap().order.id, OrderId( 1 ) );
  assert_eq!( level_pop_front( &mut level ).unwrap().order.id, OrderId( 2 ) );
  assert_eq!( level_pop_front( &mut level ).unwrap().order.id, OrderId( 3 ) );
}

#[ test ]
fn popping_an_empty_level_reports_none_rather_than_panicking()
{
  let mut level = level_new( Price::parse( "2.50" ).unwrap() );
  assert_eq!( level_pop_front( &mut level ), None );
}

#[ test ]
fn removing_by_id_finds_a_node_wherever_it_sits()
{
  let mut level = level_new( Price::parse( "2.50" ).unwrap() );
  level_push( &mut level, node( 1, 4, 1 ) );
  level_push( &mut level, node( 2, 4, 2 ) );
  level_push( &mut level, node( 3, 4, 3 ) );

  let removed = level_remove( &mut level, OrderId( 2 ) ).unwrap();
  assert_eq!( removed.order.id, OrderId( 2 ) );
  assert_eq!( level_len( &level ), 2 );

  // The two survivors keep their original relative order — removing the
  // middle arrival does not disturb the other two.
  assert_eq!( level_pop_front( &mut level ).unwrap().order.id, OrderId( 1 ) );
  assert_eq!( level_pop_front( &mut level ).unwrap().order.id, OrderId( 3 ) );
}

#[ test ]
fn removing_an_absent_id_reports_none_rather_than_panicking()
{
  let mut level = level_new( Price::parse( "2.50" ).unwrap() );
  level_push( &mut level, node( 1, 4, 1 ) );

  assert_eq!( level_remove( &mut level, OrderId( 99 ) ), None );
  assert_eq!( level_len( &level ), 1 );
}

#[ test ]
fn qty_sum_adds_every_resting_nodes_remaining()
{
  let mut level = level_new( Price::parse( "2.50" ).unwrap() );
  level_push( &mut level, node( 1, 4, 1 ) );
  level_push( &mut level, node( 2, 7, 2 ) );

  assert_eq!( level_qty_sum( &level ), Ok( Quantity::from_int( 11 ).unwrap() ) );
}

#[ test ]
fn qty_sum_of_an_empty_level_is_zero()
{
  let level = level_new( Price::parse( "2.50" ).unwrap() );
  assert_eq!( level_qty_sum( &level ), Ok( Quantity::ZERO ) );
}

/// Two orders each at the quantity ceiling sum past it — reported, not a
/// panic.
#[ test ]
fn qty_sum_past_the_ceiling_is_an_error()
{
  let mut level = level_new( Price::parse( "2.50" ).unwrap() );
  level_push( &mut level, LevelNode { remaining : Quantity::MAX, ..node( 1, 1, 1 ) } );
  level_push( &mut level, LevelNode { remaining : Quantity::MAX, ..node( 2, 1, 2 ) } );

  assert!( level_qty_sum( &level ).is_err() );
}

#[ test ]
fn a_level_is_empty_again_once_its_last_node_is_popped()
{
  let mut level = level_new( Price::parse( "2.50" ).unwrap() );
  level_push( &mut level, node( 1, 4, 1 ) );
  assert!( !level_empty_is( &level ) );

  level_pop_front( &mut level );
  assert!( level_empty_is( &level ) );
}

/// `nodes` is public specifically so a book can reduce the front node's
/// `remaining` in place without a pop/push round trip — this pins that the
/// field actually supports it.
#[ test ]
fn the_front_nodes_remaining_can_be_reduced_in_place()
{
  let mut level = level_new( Price::parse( "2.50" ).unwrap() );
  level_push( &mut level, node( 1, 4, 1 ) );

  let front = level.nodes.first_mut().unwrap();
  front.remaining = front.remaining.checked_sub( Quantity::from_int( 1 ).unwrap() ).unwrap();

  assert_eq!( level.nodes[ 0 ].remaining, Quantity::from_int( 3 ).unwrap() );
  assert_eq!( level.nodes[ 0 ].order.id, OrderId( 1 ), "reducing remaining must not disturb identity" );
}
