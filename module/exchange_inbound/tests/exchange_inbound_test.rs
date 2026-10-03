//! Test Matrix T01 — ring round-trip (flush/drain/overflow) and T02 —
//! `inbound_apply`'s dispatch to `exchange_match`/`exchange_rest`.

use exact_arith::{ Money, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_inbound::{ inbound_apply, inbound_drain, inbound_flush, inbound_overflow_reject, inbound_ring, InboundCmd, InboundOutcome };
use exchange_order::Order;
use exchange_rest::RestReplaceError;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_stp::SelfMatchPolicy;
use exchange_tif::Tif;

const INSTRUMENT : InstrumentId = InstrumentId( 1 );

fn resting( id : u64, side : Side, price : &str, quantity : i64, tif : Tif ) -> Resting
{
  let quantity = Quantity::from_int( quantity ).unwrap();
  Resting
  {
    order : Order
    {
      id : OrderId( id ), instrument : INSTRUMENT, account : AccountId( id ),
      side, price : Money::parse( price ).unwrap(), quantity, tif,
    },
    remaining : quantity,
    arrival : Sequence( id ),
  }
}

// ── T01: ring round-trip ────────────────────────────────────────────────

#[ test ]
fn a_pushed_command_drains_in_the_same_order_it_was_pushed()
{
  let mut split = inbound_ring( 8 ).unwrap();
  let mut ends = split.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let a = InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) };
  let b = InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 2 ) };
  producer.try_push( a ).unwrap();
  producer.try_push( b ).unwrap();

  assert_eq!( inbound_drain( &mut consumer ), vec![ a, b ], "FIFO — push order must survive the ring" );
}

#[ test ]
fn flush_accepts_as_many_as_fit_and_reports_the_count()
{
  let mut split = inbound_ring( 2 ).unwrap();
  let mut ends = split.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let cmds = ( 1..=4u64 ).map( | id | InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( id ) } );
  let accepted = inbound_flush( &mut producer, cmds );

  assert_eq!( accepted, 2, "capacity 2 — only the first two of four fit" );
  assert_eq!( inbound_drain( &mut consumer ).len(), 2 );
}

#[ test ]
fn a_full_ring_rejects_the_next_publish_instead_of_dropping_it()
{
  let mut split = inbound_ring( 2 ).unwrap();
  let mut ends = split.ends();
  let ( mut producer, _consumer ) = ends.split();

  producer.try_push( InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) } ).unwrap();
  producer.try_push( InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 2 ) } ).unwrap();

  let third = InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 3 ) };
  let rejected = inbound_overflow_reject( &mut producer, third );

  assert_eq!( rejected, Err( third ), "the ring must hand the rejected command back, not silently discard it" );
}

#[ test ]
fn drain_is_empty_on_a_fresh_ring()
{
  let mut split = inbound_ring( 4 ).unwrap();
  let mut ends = split.ends();
  let ( _producer, mut consumer ) = ends.split();

  assert!( inbound_drain( &mut consumer ).is_empty() );
}

// ── T02: inbound_apply dispatch ─────────────────────────────────────────

#[ test ]
fn a_place_with_nothing_to_cross_rests_on_the_book()
{
  let mut book = Book::new();
  let cmd = InboundCmd::Place( resting( 1, Side::Buy, "1.00", 5, Tif::Gtc ) );

  let outcome = inbound_apply( &mut book, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  let InboundOutcome::Crossed( crossing ) = outcome else { panic!( "Place must produce Crossed" ) };
  assert!( crossing.trades.is_empty(), "nothing on the book to cross against" );
  assert_eq!( book.best( INSTRUMENT, Side::Buy ).unwrap().order.id, OrderId( 1 ) );
}

#[ test ]
fn a_place_that_fully_crosses_leaves_nothing_resting()
{
  let mut book = Book::new();
  assert!( book.insert( resting( 1, Side::Sell, "1.00", 5, Tif::Gtc ) ), "fresh id, must succeed" );

  let cmd = InboundCmd::Place( resting( 2, Side::Buy, "1.00", 5, Tif::Gtc ) );
  let outcome = inbound_apply( &mut book, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  let InboundOutcome::Crossed( crossing ) = outcome else { panic!( "Place must produce Crossed" ) };
  assert!( crossing.is_complete() );
  assert_eq!( crossing.trades.len(), 1 );
  assert!( book.is_empty(), "both sides of the full fill are gone" );
}

#[ test ]
fn an_ioc_place_never_rests_its_remainder()
{
  let mut book = Book::new();
  let cmd = InboundCmd::Place( resting( 1, Side::Buy, "1.00", 5, Tif::Ioc ) );

  let outcome = inbound_apply( &mut book, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  let InboundOutcome::Crossed( crossing ) = outcome else { panic!( "Place must produce Crossed" ) };
  assert_eq!( crossing.trades.len(), 0 );
  assert!( !crossing.is_complete(), "nothing to cross against, so the whole quantity is unfilled" );
  assert!( book.is_empty(), "IOC must not rest the unfilled remainder" );
}

#[ test ]
fn cancel_withdraws_a_resting_order_and_reports_none_when_absent()
{
  let mut book = Book::new();
  assert!( book.insert( resting( 1, Side::Buy, "1.00", 5, Tif::Gtc ) ), "fresh id, must succeed" );

  let found = inbound_apply( &mut book, SelfMatchPolicy::CancelResting, InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) } ).unwrap();
  assert!( matches!( found, InboundOutcome::Cancelled( Some( _ ) ) ) );

  let missing = inbound_apply( &mut book, SelfMatchPolicy::CancelResting, InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( 1 ) } ).unwrap();
  assert!( matches!( missing, InboundOutcome::Cancelled( None ) ), "already withdrawn — a race result, not an error" );
}

#[ test ]
fn replace_swaps_the_resting_order_atomically()
{
  let mut book = Book::new();
  assert!( book.insert( resting( 1, Side::Sell, "2.00", 4, Tif::Gtc ) ), "fresh id, must succeed" );

  let cmd = InboundCmd::Replace { instrument : INSTRUMENT, old_id : OrderId( 1 ), new_resting : resting( 1, Side::Sell, "2.10", 6, Tif::Gtc ) };
  let outcome = inbound_apply( &mut book, SelfMatchPolicy::CancelResting, cmd ).unwrap();

  let InboundOutcome::Replaced( Ok( old ) ) = outcome else { panic!( "Replace must succeed here" ) };
  assert_eq!( old.order.price, Money::parse( "2.00" ).unwrap() );
  assert_eq!( book.best( INSTRUMENT, Side::Sell ).unwrap().order.price, Money::parse( "2.10" ).unwrap() );
}

#[ test ]
fn replace_of_a_missing_order_reports_missing()
{
  let mut book = Book::new();
  let cmd = InboundCmd::Replace { instrument : INSTRUMENT, old_id : OrderId( 99 ), new_resting : resting( 99, Side::Sell, "2.10", 6, Tif::Gtc ) };

  let outcome = inbound_apply( &mut book, SelfMatchPolicy::CancelResting, cmd ).unwrap();
  assert!( matches!( outcome, InboundOutcome::Replaced( Err( RestReplaceError::Missing ) ) ) );
}
