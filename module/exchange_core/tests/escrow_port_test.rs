//! The facade's own sequencing, graded against a scripted [`EscrowPort`].
//!
//! `submission_test.rs` drives the real `Escrow`, so every escrow failure it
//! covers had to be engineered out of real balances first. Here escrow is a
//! fake that holds no balances at all: it records the calls applied to it and
//! fails exactly where a test names an order id. What is left under test is
//! only what `exchange_core` itself decides — which escrow call comes when,
//! and that nothing commits once any of them refuses.

use std::collections::BTreeMap;

use exchange_core::
{
  AccountId, Consumer, EscrowError, EscrowPort, EventKind, Exchange, ExchangeError, InboundCmd, InstrumentId,
  Obligation, Order, OrderId, Price, Producer, Quantity, Receipt, RejectReason, Resting, SelfMatchPolicy,
  Sequence, Side, StepOutcome, Tif, Trade, inbound_flush, inbound_ring, obligation,
};

const INSTRUMENT : InstrumentId = InstrumentId( 1 );

fn qty( whole : i64 ) -> Quantity
{
  Quantity::from_int( whole ).unwrap()
}

/// One escrow call that took effect.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
enum Call
{
  Reserve( OrderId ),
  Release( OrderId ),
  Settle { taker : OrderId, maker : OrderId },
}

impl Call
{
  fn names( &self, id : OrderId ) -> bool
  {
    match *self
    {
      Self::Reserve( order ) | Self::Release( order ) => order == id,
      Self::Settle { taker, maker } => taker == id || maker == id,
    }
  }
}

/// An [`EscrowPort`] with no balances behind it.
///
/// `calls` belongs to each instance, and the derived `Clone` copies it, so
/// whatever the facade's dry run does to its clone never reaches the original
/// — `exchange.escrow().calls` is exactly what was committed. A refused call
/// is not recorded: per the port's own contract, it changed nothing.
///
/// Failures are keyed by order id, never by call count. A count would make
/// the dry-run clone and the real replay disagree about which call fails,
/// which is the one thing the port's determinism rule forbids.
///
/// Amounts are not modelled: `release` hands back what `reserve` stored,
/// however much `settle` has since consumed.
#[ derive( Debug, Clone, Default ) ]
struct ScriptedEscrow
{
  calls : Vec< Call >,
  reservations : BTreeMap< OrderId, Obligation >,
  reserve_fails : BTreeMap< OrderId, EscrowError >,
  release_fails : BTreeMap< OrderId, EscrowError >,
  settle_fails_for_maker : BTreeMap< OrderId, EscrowError >,
}

impl EscrowPort for ScriptedEscrow
{
  fn reserve( &mut self, order : &Order ) -> Result< Obligation, EscrowError >
  {
    if let Some( error ) = self.reserve_fails.get( &order.id )
    {
      return Err( *error );
    }
    let obligation = obligation( order )?;
    self.reservations.insert( order.id, obligation );
    self.calls.push( Call::Reserve( order.id ) );
    Ok( obligation )
  }

  fn release( &mut self, _owner : AccountId, order : OrderId ) -> Result< Obligation, EscrowError >
  {
    if let Some( error ) = self.release_fails.get( &order )
    {
      return Err( *error );
    }
    let obligation = self.reservations.remove( &order ).ok_or( EscrowError::NoReservation( order ) )?;
    self.calls.push( Call::Release( order ) );
    Ok( obligation )
  }

  fn settle( &mut self, trade : &Trade, _taker_side : Side, _taker_limit : Price ) -> Result< (), EscrowError >
  {
    if let Some( error ) = self.settle_fails_for_maker.get( &trade.maker )
    {
      return Err( *error );
    }
    self.calls.push( Call::Settle { taker : trade.taker, maker : trade.maker } );
    Ok( () )
  }
}

fn order( account : AccountId, side : Side, price : &str, quantity : i64, tif : Tif ) -> Order
{
  Order { id : OrderId( 0 ), instrument : INSTRUMENT, account, side, price : Price::parse( price ).unwrap(), quantity : qty( quantity ), tif, client : None }
}

/// Push one `Place` through the ring and apply it. `order.id` is a
/// placeholder — the facade assigns ids itself, from 0, in arrival order.
fn place
(
  exchange : &mut Exchange< ScriptedEscrow >,
  producer : &mut Producer< '_, InboundCmd >,
  consumer : &mut Consumer< '_, InboundCmd >,
  order : Order,
  policy : SelfMatchPolicy,
) -> Result< Receipt, ExchangeError >
{
  let resting = Resting { order, remaining : order.quantity, arrival : Sequence( 0 ) };
  assert_eq!( inbound_flush( producer, [ InboundCmd::Place( resting ) ] ), 1 );

  match exchange.exchange_step( consumer, policy ).into_iter().next()
  {
    Some( StepOutcome::Placed( result ) ) => result,
    other => panic!( "place() only ever pushes Place — got {other:?}" ),
  }
}

/// Every [`EscrowError`] a reservation can refuse with reaches the caller as
/// the [`RejectReason`] the facade documents for it, with nothing committed.
#[ test ]
fn every_reserve_failure_maps_to_its_reject_reason()
{
  let table =
  [
    ( EscrowError::UnknownAccount( AccountId( 1 ) ), RejectReason::UnknownAccount ),
    ( EscrowError::Obligation( exchange_core::TypeError::NotionalInexact ), RejectReason::ObligationUnrepresentable ),
    ( EscrowError::Arithmetic, RejectReason::ReservationUnrepresentable ),
    ( EscrowError::Insufficient, RejectReason::InsufficientFunds ),
    ( EscrowError::NotReserved, RejectReason::InsufficientFunds ),
    ( EscrowError::NoReservation( OrderId( 0 ) ), RejectReason::InsufficientFunds ),
    ( EscrowError::AlreadyReserved( OrderId( 0 ) ), RejectReason::InsufficientFunds ),
    ( EscrowError::ObligationMismatch, RejectReason::InsufficientFunds ),
    ( EscrowError::NegativeAmount, RejectReason::InsufficientFunds ),
  ];

  for ( error, reason ) in table
  {
    let escrow = ScriptedEscrow { reserve_fails : BTreeMap::from( [ ( OrderId( 0 ), error ) ] ), ..ScriptedEscrow::default() };
    let mut exchange = Exchange::with_escrow( escrow );
    let mut ring = inbound_ring( 8 ).unwrap();
    let mut ends = ring.ends();
    let ( mut producer, mut consumer ) = ends.split();

    let placed = place( &mut exchange, &mut producer, &mut consumer, order( AccountId( 1 ), Side::Buy, "1.00", 2, Tif::Gtc ), SelfMatchPolicy::CancelIncoming );

    assert_eq!( placed, Err( ExchangeError::Rejected( reason ) ), "{error:?}" );
    assert_eq!( exchange.events().len(), 1, "{error:?}: only the rejection is recorded" );
    assert_eq!( exchange.events()[ 0 ].kind, EventKind::OrderRejected { reason }, "{error:?}" );
    assert!( exchange.escrow().calls.is_empty(), "{error:?}: nothing reached the real escrow" );
    assert!( exchange.book().is_empty(), "{error:?}: nothing reached the book" );
  }
}

/// A sweep whose second fill cannot settle commits none of the first: the
/// book, the escrow, and the record all read as if the order never arrived,
/// apart from its own rejection.
#[ test ]
fn a_settle_failure_mid_crossing_commits_nothing()
{
  let escrow = ScriptedEscrow { settle_fails_for_maker : BTreeMap::from( [ ( OrderId( 1 ), EscrowError::Arithmetic ) ] ), ..ScriptedEscrow::default() };
  let mut exchange = Exchange::with_escrow( escrow );
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  place( &mut exchange, &mut producer, &mut consumer, order( AccountId( 1 ), Side::Sell, "1.00", 2, Tif::Gtc ), SelfMatchPolicy::CancelIncoming ).unwrap();
  place( &mut exchange, &mut producer, &mut consumer, order( AccountId( 1 ), Side::Sell, "1.10", 2, Tif::Gtc ), SelfMatchPolicy::CancelIncoming ).unwrap();
  let book = exchange.book().clone();
  let calls = exchange.escrow().calls.clone();
  let events = exchange.events().len();

  let sweep = place( &mut exchange, &mut producer, &mut consumer, order( AccountId( 2 ), Side::Buy, "1.10", 4, Tif::Gtc ), SelfMatchPolicy::CancelIncoming );

  assert_eq!( sweep, Err( ExchangeError::Rejected( RejectReason::ReservationUnrepresentable ) ) );
  assert_eq!( exchange.book(), &book, "the first maker must still rest in full" );
  assert_eq!( exchange.escrow().calls, calls, "not even the first fill may settle for real" );
  assert_eq!( exchange.events().len(), events + 1 );
  assert!( matches!( exchange.events()[ events ].kind, EventKind::OrderRejected { .. } ) );
}

/// A self-match cancellation whose release refuses commits nothing either —
/// the resting order it would have withdrawn is still there afterwards.
#[ test ]
fn a_release_failure_on_self_match_commits_nothing()
{
  let escrow = ScriptedEscrow { release_fails : BTreeMap::from( [ ( OrderId( 0 ), EscrowError::Arithmetic ) ] ), ..ScriptedEscrow::default() };
  let mut exchange = Exchange::with_escrow( escrow );
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  place( &mut exchange, &mut producer, &mut consumer, order( AccountId( 1 ), Side::Sell, "1.00", 2, Tif::Gtc ), SelfMatchPolicy::CancelResting ).unwrap();
  let book = exchange.book().clone();
  let calls = exchange.escrow().calls.clone();
  let events = exchange.events().len();

  let own = place( &mut exchange, &mut producer, &mut consumer, order( AccountId( 1 ), Side::Buy, "1.00", 2, Tif::Gtc ), SelfMatchPolicy::CancelResting );

  assert_eq!( own, Err( ExchangeError::Rejected( RejectReason::ReservationUnrepresentable ) ) );
  assert_eq!( exchange.book(), &book, "the resting order self-match would have withdrawn must still rest" );
  assert_eq!( exchange.escrow().calls, calls );
  assert_eq!( exchange.events().len(), events + 1 );
  assert!( matches!( exchange.events()[ events ].kind, EventKind::OrderRejected { .. } ) );
}

/// An IOC taker reserves, settles what it can, and only then releases its
/// unfilled remainder — the reservation is never left behind and never
/// returned early.
#[ test ]
fn an_ioc_remainder_releases_after_its_settles()
{
  let mut exchange = Exchange::with_escrow( ScriptedEscrow::default() );
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  place( &mut exchange, &mut producer, &mut consumer, order( AccountId( 1 ), Side::Sell, "1.00", 2, Tif::Gtc ), SelfMatchPolicy::CancelIncoming ).unwrap();
  let taking = place( &mut exchange, &mut producer, &mut consumer, order( AccountId( 2 ), Side::Buy, "1.00", 5, Tif::Ioc ), SelfMatchPolicy::CancelIncoming ).unwrap();

  assert!( taking.tif_dropped );
  let taker = taking.order;
  let taker_calls : Vec< Call > = exchange.escrow().calls.iter().copied().filter( | call | call.names( taker ) ).collect();
  assert_eq!
  (
    taker_calls,
    [ Call::Reserve( taker ), Call::Settle { taker, maker : OrderId( 0 ) }, Call::Release( taker ) ],
  );
}

/// No escrow call commits without the event that records it: every
/// reservation has its `OrderAccepted`, every settlement its `Trade`, every
/// release its `OrderCancelled` — across resting, crossing, an IOC drop, a
/// self-match, and a cancel.
#[ test ]
fn every_applied_escrow_call_has_its_event()
{
  let mut exchange = Exchange::with_escrow( ScriptedEscrow::default() );
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  let mut run = | order, policy | place( &mut exchange, &mut producer, &mut consumer, order, policy ).unwrap();

  run( order( AccountId( 1 ), Side::Sell, "1.00", 2, Tif::Gtc ), SelfMatchPolicy::CancelIncoming );
  run( order( AccountId( 2 ), Side::Buy, "1.00", 1, Tif::Gtc ), SelfMatchPolicy::CancelIncoming );
  run( order( AccountId( 2 ), Side::Buy, "1.00", 3, Tif::Ioc ), SelfMatchPolicy::CancelIncoming );
  run( order( AccountId( 1 ), Side::Sell, "2.00", 2, Tif::Gtc ), SelfMatchPolicy::CancelIncoming );
  let own = run( order( AccountId( 1 ), Side::Buy, "2.00", 1, Tif::Gtc ), SelfMatchPolicy::CancelIncoming );
  assert!( own.self_match_cancelled );
  exchange.cancel( OrderId( 3 ) ).unwrap();

  let calls = &exchange.escrow().calls;
  let count_calls = | pick : fn( &Call ) -> bool | calls.iter().filter( | call | pick( call ) ).count();
  let count_events = | pick : fn( &EventKind ) -> bool | exchange.events().iter().filter( | event | pick( &event.kind ) ).count();

  let reserves = count_calls( | call | matches!( call, Call::Reserve( _ ) ) );
  let settles = count_calls( | call | matches!( call, Call::Settle { .. } ) );
  let releases = count_calls( | call | matches!( call, Call::Release( _ ) ) );
  assert_eq!( ( reserves, settles, releases ), ( 5, 2, 3 ), "the scenario itself: five placements, two fills, IOC + self-match + cancel" );

  assert_eq!( count_events( | kind | matches!( kind, EventKind::OrderAccepted { .. } ) ), reserves );
  assert_eq!( count_events( | kind | matches!( kind, EventKind::Trade( _ ) ) ), settles );
  assert_eq!( count_events( | kind | matches!( kind, EventKind::OrderCancelled { .. } ) ), releases );
}

/// A cancel whose release refuses leaves the order resting, reports the
/// escrow refusal, and records no cancellation.
#[ test ]
fn a_cancel_whose_release_fails_leaves_the_order_resting()
{
  let escrow = ScriptedEscrow { release_fails : BTreeMap::from( [ ( OrderId( 0 ), EscrowError::Arithmetic ) ] ), ..ScriptedEscrow::default() };
  let mut exchange = Exchange::with_escrow( escrow );
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  place( &mut exchange, &mut producer, &mut consumer, order( AccountId( 1 ), Side::Sell, "1.00", 2, Tif::Gtc ), SelfMatchPolicy::CancelIncoming ).unwrap();

  assert_eq!( exchange.cancel( OrderId( 0 ) ), Err( ExchangeError::Escrow( EscrowError::Arithmetic ) ) );
  assert!( exchange.book().iter().any( | resting | resting.order.id == OrderId( 0 ) ), "the order must still rest" );
  assert!( !exchange.escrow().calls.contains( &Call::Release( OrderId( 0 ) ) ) );
  assert!( !exchange.events().iter().any( | event | matches!( event.kind, EventKind::OrderCancelled { .. } ) ) );
}
