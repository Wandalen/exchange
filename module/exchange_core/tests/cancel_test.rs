//! Who may cancel what (ADR-003).
//!
//! A cancel arrives only through the ring. It names an account, an
//! instrument, and the order by either id: the one the exchange assigned, or
//! the account's own client id. It withdraws the order only when all three
//! match. Every miss — never placed, already gone, another account's, another
//! instrument's — answers the same `NotResting`, and changes nothing.

use exchange_core::
{
  AccountId, AssetId, CancelCause, ClientOrderId, Consumer, EventKind, Exchange, ExchangeError, InboundCmd, InstrumentId, Money,
  Obligation, Order, OrderId, OrderRef, Price, Producer, Quantity, Receipt, Resting, SelfMatchPolicy, Sequence, Side, StepOutcome,
  Tif, inbound_flush, inbound_ring,
};

const INSTRUMENT : InstrumentId = InstrumentId( 1 );
const OWNER : AccountId = AccountId( 1 );
const OTHER : AccountId = AccountId( 2 );

fn money( text : &str ) -> Money
{
  Money::parse( text ).unwrap()
}

fn qty( whole : i64 ) -> Quantity
{
  Quantity::from_int( whole ).unwrap()
}

fn market() -> Exchange
{
  let mut exchange = Exchange::new();
  exchange.open_account( OWNER, money( "1000" ), qty( 100 ) ).unwrap();
  exchange.open_account( OTHER, money( "1000" ), qty( 100 ) ).unwrap();
  exchange.spec_register( INSTRUMENT, AssetId( 1 ), AssetId( 2 ), Price::parse( "0.01" ).unwrap(), qty( 1 ) ).unwrap();
  exchange
}

/// A resting buy of 10 at 2.50, reserving 25.00, under `client` if given.
fn bid( account : AccountId, client : Option< u64 > ) -> Order
{
  Order
  {
    id : OrderId( 0 ), instrument : INSTRUMENT, account, side : Side::Buy, price : Price::parse( "2.50" ).unwrap(),
    quantity : qty( 10 ), tif : Tif::Gtc, client : client.map( ClientOrderId ),
  }
}

fn place
(
  exchange : &mut Exchange,
  producer : &mut Producer< '_, InboundCmd >,
  consumer : &mut Consumer< '_, InboundCmd >,
  order : Order,
) -> Receipt
{
  let resting = Resting { order, remaining : order.quantity, arrival : Sequence( 0 ) };
  assert_eq!( inbound_flush( producer, &[ InboundCmd::Place( resting ) ] ), 1 );
  match exchange.exchange_step( consumer, SelfMatchPolicy::CancelIncoming ).into_iter().next()
  {
    Some( StepOutcome::Placed( Ok( receipt ) ) ) => receipt,
    other => panic!( "the placement must succeed — got {other:?}" ),
  }
}

fn cancel
(
  exchange : &mut Exchange,
  producer : &mut Producer< '_, InboundCmd >,
  consumer : &mut Consumer< '_, InboundCmd >,
  instrument : InstrumentId,
  account : AccountId,
  order : OrderRef,
) -> Result< Quantity, ExchangeError >
{
  assert_eq!( inbound_flush( producer, &[ InboundCmd::Cancel { instrument, account, order } ] ), 1 );
  match exchange.exchange_step( consumer, SelfMatchPolicy::CancelIncoming ).into_iter().next()
  {
    Some( StepOutcome::Cancelled( result ) ) => result,
    other => panic!( "cancel() only pushes Cancel — got {other:?}" ),
  }
}

/// The last event is the owner's cancel of the whole 10, returning 25.00.
fn assert_cancelled_on_record( exchange : &Exchange )
{
  let last = exchange.events().last().unwrap();
  assert_eq!
  (
    last.kind,
    EventKind::OrderCancelled { cause : CancelCause::Request, quantity : qty( 10 ), released : Obligation::Cash( money( "25" ) ) },
  );
  assert_eq!( last.account, OWNER );
  assert_eq!( exchange.escrow().account( OWNER ).unwrap().cash.reserved(), Money::ZERO );
  assert!( exchange.book().is_empty() );
  assert_eq!( exchange.stats_get().cancels, 1 );
}

#[ test ]
fn an_owner_cancel_by_order_id_releases_and_emits_request()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  let resting = place( &mut exchange, &mut producer, &mut consumer, bid( OWNER, None ) );

  let withdrawn = cancel( &mut exchange, &mut producer, &mut consumer, INSTRUMENT, OWNER, OrderRef::Id( resting.order ) );

  assert_eq!( withdrawn, Ok( qty( 10 ) ) );
  assert_cancelled_on_record( &exchange );
}

/// A producer never sees the id the exchange assigned; its own client id is
/// enough.
#[ test ]
fn an_owner_cancel_by_client_id_releases_and_emits_request()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  place( &mut exchange, &mut producer, &mut consumer, bid( OWNER, Some( 7 ) ) );

  let withdrawn = cancel( &mut exchange, &mut producer, &mut consumer, INSTRUMENT, OWNER, OrderRef::Client( ClientOrderId( 7 ) ) );

  assert_eq!( withdrawn, Ok( qty( 10 ) ) );
  assert_cancelled_on_record( &exchange );
}

/// Another account's cancel, by either id, answers as if the order did not
/// exist, and moves nothing: book, escrow, log and stats stay as they were.
#[ test ]
fn a_cancel_by_another_account_is_not_resting_and_changes_nothing()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  let resting = place( &mut exchange, &mut producer, &mut consumer, bid( OWNER, Some( 7 ) ) );
  let ( book, escrow, events, stats ) = ( exchange.book().clone(), exchange.escrow().clone(), exchange.events().len(), exchange.stats_get() );

  for order in [ OrderRef::Id( resting.order ), OrderRef::Client( ClientOrderId( 7 ) ) ]
  {
    let refused = cancel( &mut exchange, &mut producer, &mut consumer, INSTRUMENT, OTHER, order );
    assert_eq!( refused, Err( ExchangeError::NotResting( order ) ) );
  }

  assert_eq!( exchange.book(), &book );
  assert_eq!( exchange.escrow(), &escrow );
  assert_eq!( exchange.events().len(), events );
  assert_eq!( exchange.stats_get(), stats );
}

/// A client id is the account's own: two accounts may use the same value, and
/// each cancel reaches only its own order.
#[ test ]
fn a_client_id_names_only_its_own_accounts_order()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  let owners = place( &mut exchange, &mut producer, &mut consumer, bid( OWNER, Some( 7 ) ) );
  let others = place( &mut exchange, &mut producer, &mut consumer, bid( OTHER, Some( 7 ) ) );

  cancel( &mut exchange, &mut producer, &mut consumer, INSTRUMENT, OTHER, OrderRef::Client( ClientOrderId( 7 ) ) ).unwrap();

  let left : Vec< _ > = exchange.book().iter().map( | resting | resting.order.id ).collect();
  assert_eq!( left, [ owners.order ] );
  assert_ne!( owners.order, others.order );
}

/// The order rests on one instrument; a cancel naming another withdraws
/// nothing.
#[ test ]
fn a_cancel_naming_the_wrong_instrument_is_not_resting()
{
  let mut exchange = market();
  let other = InstrumentId( 2 );
  exchange.spec_register( other, AssetId( 1 ), AssetId( 2 ), Price::parse( "0.01" ).unwrap(), qty( 1 ) ).unwrap();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  let resting = place( &mut exchange, &mut producer, &mut consumer, bid( OWNER, None ) );

  let refused = cancel( &mut exchange, &mut producer, &mut consumer, other, OWNER, OrderRef::Id( resting.order ) );

  assert_eq!( refused, Err( ExchangeError::NotResting( OrderRef::Id( resting.order ) ) ) );
  assert_eq!( exchange.book().len(), 1 );
}

/// The second cancel of the same order finds nothing and releases nothing.
#[ test ]
fn a_repeat_cancel_is_not_resting_and_releases_nothing()
{
  let mut exchange = market();
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  let resting = place( &mut exchange, &mut producer, &mut consumer, bid( OWNER, Some( 7 ) ) );
  cancel( &mut exchange, &mut producer, &mut consumer, INSTRUMENT, OWNER, OrderRef::Id( resting.order ) ).unwrap();
  let escrow = exchange.escrow().clone();

  for order in [ OrderRef::Id( resting.order ), OrderRef::Client( ClientOrderId( 7 ) ) ]
  {
    let again = cancel( &mut exchange, &mut producer, &mut consumer, INSTRUMENT, OWNER, order );
    assert_eq!( again, Err( ExchangeError::NotResting( order ) ) );
  }

  assert_eq!( exchange.escrow(), &escrow );
  assert_cancelled_on_record( &exchange );
}
