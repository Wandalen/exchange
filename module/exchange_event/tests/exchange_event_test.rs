//! Test Matrix T01 — the drain takes ownership and empties the source.

use exchange_event::{ Event, EventKind, event_clear, event_drain, event_len, event_push };
use exchange_types::{ AccountId, OrderId, RejectReason, Sequence };

fn rejected( id : u64 ) -> Event
{
  Event
  {
    sequence : Sequence( id ),
    order : OrderId( id ),
    account : AccountId( id ),
    kind : EventKind::OrderRejected { reason : RejectReason::ZeroQuantity },
  }
}

/// T01 — draining an empty source yields nothing and leaves it empty.
#[ test ]
fn draining_empty_yields_nothing()
{
  let mut events : Vec< Event > = Vec::new();
  assert_eq!( event_drain( &mut events ), Vec::new() );
  assert!( events.is_empty() );
}

/// T01 — draining takes every event, in order, and leaves the source empty.
#[ test ]
fn drain_takes_every_event_in_order_and_empties_the_source()
{
  let mut events = vec![ rejected( 1 ), rejected( 2 ), rejected( 3 ) ];
  let drained = event_drain( &mut events );

  assert_eq!( drained.len(), 3 );
  assert_eq!( drained[ 0 ].sequence, Sequence( 1 ) );
  assert_eq!( drained[ 2 ].sequence, Sequence( 3 ) );
  assert!( events.is_empty(), "the source must be left empty, not merely copied from" );
}

/// A second drain after the first returns nothing — the first drain actually
/// took ownership rather than leaving a copy behind.
#[ test ]
fn a_second_drain_after_the_first_returns_nothing()
{
  let mut events = vec![ rejected( 1 ) ];
  let _ = event_drain( &mut events );

  assert_eq!( event_drain( &mut events ), Vec::new() );
}

/// T01 — `event_push` actually appends, `event_len` reflects it.
#[ test ]
fn push_appends_and_len_reflects_it()
{
  let mut events : Vec< Event > = Vec::new();
  assert_eq!( event_len( &events ), 0 );

  event_push( &mut events, rejected( 1 ) );
  event_push( &mut events, rejected( 2 ) );

  assert_eq!( event_len( &events ), 2 );
  assert_eq!( events[ 1 ].sequence, Sequence( 2 ) );
}

/// T01 — `event_clear` empties the source without returning anything.
#[ test ]
fn clear_empties_without_returning()
{
  let mut events = vec![ rejected( 1 ), rejected( 2 ) ];
  event_clear( &mut events );

  assert!( events.is_empty() );
  assert_eq!( event_len( &events ), 0 );
}
