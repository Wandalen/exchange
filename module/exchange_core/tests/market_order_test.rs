//! A market order is a limit order with a cap, sent IOC or FOK.
//!
//! v0.1 has no order type for "market". A market buy is
//! `Order { price: cap, tif: Ioc | Fok }`: it reserves `cap × quantity`, pays
//! each maker's own price, never trades above the cap, and gets back what it
//! did not spend. A market sell is the same with `price: 0`, which crosses
//! every bid. Nothing enforces the IOC or FOK: a zero-price sell sent GTC
//! rests, and the next buy takes it for nothing.
//!
//! These tests pin that shape through the facade, on a grid where
//! one tick times one lot is exact: on an inexact grid matching declines a
//! fill whose notional needs a seventh decimal, and a market buy can end
//! unfilled with asks inside its cap.

use exchange_core::
{
  AccountId, AssetId, CancelCause, Consumer, EventKind, Exchange, ExchangeError, InboundCmd, InstrumentId, Money, Obligation, Order, OrderId, Price,
  Producer, Quantity, Receipt, RejectReason, Resting, SelfMatchPolicy, Sequence, Side, StepOutcome, Tif, inbound_flush, inbound_ring,
};

const INSTRUMENT : InstrumentId = InstrumentId( 1 );
const MAKER : AccountId = AccountId( 1 );
const TAKER : AccountId = AccountId( 2 );

fn money( text : &str ) -> Money
{
  Money::parse( text ).unwrap()
}

fn price( text : &str ) -> Price
{
  Price::parse( text ).unwrap()
}

fn qty( whole : i64 ) -> Quantity
{
  Quantity::from_int( whole ).unwrap()
}

/// Two accounts funded with `taker_cash` for the taker, on a cent tick and a
/// whole-unit lot, so every notional here is exact.
fn exchange_with( taker_cash : &str ) -> Exchange
{
  let mut exchange = Exchange::new();
  exchange.open_account( MAKER, money( "1000" ), qty( 100 ) ).unwrap();
  exchange.open_account( TAKER, money( taker_cash ), qty( 100 ) ).unwrap();
  exchange.spec_register( INSTRUMENT, AssetId( 1 ), AssetId( 2 ), price( "0.01" ), qty( 1 ) ).unwrap();
  exchange
}

fn order( account : AccountId, side : Side, limit : &str, quantity : i64, tif : Tif ) -> Order
{
  Order { id : OrderId( 0 ), instrument : INSTRUMENT, account, side, price : price( limit ), quantity : qty( quantity ), tif, client : None }
}

fn place
(
  exchange : &mut Exchange,
  producer : &mut Producer< '_, InboundCmd >,
  consumer : &mut Consumer< '_, InboundCmd >,
  order : Order,
) -> Result< Receipt, ExchangeError >
{
  let resting = Resting { order, remaining : order.quantity, arrival : Sequence( 0 ) };
  assert_eq!( inbound_flush( producer, &[ InboundCmd::Place( resting ) ] ), 1 );
  match exchange.exchange_step( consumer, SelfMatchPolicy::CancelIncoming ).into_iter().next()
  {
    Some( StepOutcome::Placed( result ) ) => result,
    other => panic!( "place() only pushes Place — got {other:?}" ),
  }
}

/// A market buy walks the asks up to its cap at each maker's price, leaves
/// the asks above the cap alone, drops its remainder and keeps no
/// reservation.
#[ test ]
fn a_market_buy_pays_maker_prices_up_to_its_cap_and_drops_the_rest()
{
  let mut exchange = exchange_with( "1000" );
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  for ( limit, quantity ) in [ ( "2.00", 2 ), ( "2.10", 2 ), ( "2.30", 5 ) ]
  {
    place( &mut exchange, &mut producer, &mut consumer, order( MAKER, Side::Sell, limit, quantity, Tif::Gtc ) ).unwrap();
  }

  let taking = place( &mut exchange, &mut producer, &mut consumer, order( TAKER, Side::Buy, "2.20", 5, Tif::Ioc ) ).unwrap();

  let fills : Vec< _ > = taking.trades.iter().map( | trade | ( trade.price, trade.quantity ) ).collect();
  assert_eq!( fills, [ ( price( "2.00" ), qty( 2 ) ), ( price( "2.10" ), qty( 2 ) ) ] );
  assert!( taking.tif_dropped );
  assert_eq!( taking.resting, Quantity::ZERO );
  assert_eq!( exchange.book().len(), 1, "the ask above the cap still rests" );

  let cash = exchange.escrow().account( TAKER ).unwrap().cash;
  assert_eq!( ( cash.available(), cash.reserved() ), ( money( "991.80" ), Money::ZERO ) );
  assert!( exchange.events().iter().any( | event | matches!
  (
    event.kind,
    EventKind::OrderCancelled { cause : CancelCause::TimeInForce, quantity, released }
      if quantity == qty( 1 ) && released == Obligation::Cash( money( "2.20" ) ),
  ) ), "the unfilled unit releases its cap, 2.20; the 0.60 below the cap came back at settlement" );
}

/// A market sell is priced at zero, so it crosses every bid, best first.
#[ test ]
fn a_market_sell_at_price_zero_fills_down_the_bids()
{
  let mut exchange = exchange_with( "1000" );
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  for ( limit, quantity ) in [ ( "2.00", 2 ), ( "1.90", 2 ) ]
  {
    place( &mut exchange, &mut producer, &mut consumer, order( MAKER, Side::Buy, limit, quantity, Tif::Gtc ) ).unwrap();
  }

  let taking = place( &mut exchange, &mut producer, &mut consumer, order( TAKER, Side::Sell, "0", 3, Tif::Ioc ) ).unwrap();

  let fills : Vec< _ > = taking.trades.iter().map( | trade | ( trade.price, trade.quantity ) ).collect();
  assert_eq!( fills, [ ( price( "2.00" ), qty( 2 ) ), ( price( "1.90" ), qty( 1 ) ) ] );
  assert!( !taking.tif_dropped, "the whole quantity filled" );
  let seller = exchange.escrow().account( TAKER ).unwrap();
  assert_eq!( ( seller.cash.available(), seller.asset.available(), seller.asset.reserved() ), ( money( "1005.90" ), qty( 97 ), Quantity::ZERO ) );
}

/// A market FOK fills in full within its cap or not at all: liquidity above
/// the cap does not count towards the whole.
#[ test ]
fn a_market_fok_fills_whole_within_its_cap_or_not_at_all()
{
  let mut exchange = exchange_with( "1000" );
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  for ( limit, quantity ) in [ ( "2.00", 2 ), ( "2.10", 2 ), ( "2.30", 5 ) ]
  {
    place( &mut exchange, &mut producer, &mut consumer, order( MAKER, Side::Sell, limit, quantity, Tif::Gtc ) ).unwrap();
  }
  let before = exchange.book().clone();

  let short = place( &mut exchange, &mut producer, &mut consumer, order( TAKER, Side::Buy, "2.20", 5, Tif::Fok ) ).unwrap();
  assert!( short.trades.is_empty() && short.tif_dropped );
  assert_eq!( exchange.book(), &before );

  let whole = place( &mut exchange, &mut producer, &mut consumer, order( TAKER, Side::Buy, "2.20", 4, Tif::Fok ) ).unwrap();
  let fills : Vec< _ > = whole.trades.iter().map( | trade | ( trade.price, trade.quantity ) ).collect();
  assert_eq!( fills, [ ( price( "2.00" ), qty( 2 ) ), ( price( "2.10" ), qty( 2 ) ) ] );
  let cash = exchange.escrow().account( TAKER ).unwrap().cash;
  assert_eq!( ( cash.available(), cash.reserved() ), ( money( "991.80" ), Money::ZERO ) );
}

/// Against an empty book a market order is accepted, then cancelled whole
/// by its time in force, with its reservation released in full.
#[ test ]
fn a_market_order_on_an_empty_book_is_accepted_then_cancelled_in_full()
{
  let mut exchange = exchange_with( "1000" );
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();

  let taking = place( &mut exchange, &mut producer, &mut consumer, order( TAKER, Side::Buy, "2.20", 5, Tif::Ioc ) ).unwrap();

  assert!( taking.trades.is_empty() && taking.tif_dropped );
  let kinds : Vec< _ > = exchange.events().iter().map( | event | event.kind ).collect();
  assert!( matches!
  (
    kinds.as_slice(),
    [ EventKind::OrderAccepted { .. }, EventKind::OrderCancelled { cause : CancelCause::TimeInForce, quantity, released, .. } ]
      if *quantity == qty( 5 ) && *released == Obligation::Cash( money( "11" ) ),
  ), "{kinds:?}" );
  let cash = exchange.escrow().account( TAKER ).unwrap().cash;
  assert_eq!( ( cash.available(), cash.reserved() ), ( money( "1000" ), Money::ZERO ) );
}

/// The cost of the cap: a market buy reserves `cap × quantity`, so it is
/// refused when the account cannot cover that, even if the fill would cost
/// less. The caller controls this through the cap it picks.
#[ test ]
fn a_market_buy_needs_cash_for_its_cap_not_for_its_fill()
{
  let mut exchange = exchange_with( "10" );
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  place( &mut exchange, &mut producer, &mut consumer, order( MAKER, Side::Sell, "1.00", 5, Tif::Gtc ) ).unwrap();

  let capped_high = place( &mut exchange, &mut producer, &mut consumer, order( TAKER, Side::Buy, "2.20", 5, Tif::Ioc ) );
  assert_eq!( capped_high, Err( ExchangeError::Rejected( RejectReason::InsufficientFunds ) ) );
  assert!( matches!( exchange.events().last().unwrap().kind, EventKind::OrderRejected { reason : RejectReason::InsufficientFunds } ) );
  let cash = exchange.escrow().account( TAKER ).unwrap().cash;
  assert_eq!( ( cash.available(), cash.reserved() ), ( money( "10" ), Money::ZERO ), "a refused buy reserves nothing" );
  assert_eq!( exchange.book().len(), 1 );

  let capped_low = place( &mut exchange, &mut producer, &mut consumer, order( TAKER, Side::Buy, "2.00", 5, Tif::Ioc ) ).unwrap();
  let fills : Vec< _ > = capped_low.trades.iter().map( | trade | ( trade.price, trade.quantity ) ).collect();
  assert_eq!( fills, [ ( price( "1.00" ), qty( 5 ) ) ] );
  let cash = exchange.escrow().account( TAKER ).unwrap().cash;
  assert_eq!( ( cash.available(), cash.reserved() ), ( money( "5" ), Money::ZERO ), "the 5.00 below the cap came back" );
}

/// A market FOK sell fills in full down the bids or not at all, and a
/// killed one keeps none of its asset reserved.
#[ test ]
fn a_market_fok_sell_fills_whole_or_not_at_all()
{
  let mut exchange = exchange_with( "1000" );
  let mut ring = inbound_ring( 8 ).unwrap();
  let mut ends = ring.ends();
  let ( mut producer, mut consumer ) = ends.split();
  for ( limit, quantity ) in [ ( "2.00", 2 ), ( "1.90", 2 ) ]
  {
    place( &mut exchange, &mut producer, &mut consumer, order( MAKER, Side::Buy, limit, quantity, Tif::Gtc ) ).unwrap();
  }
  let before = exchange.book().clone();

  let short = place( &mut exchange, &mut producer, &mut consumer, order( TAKER, Side::Sell, "0", 5, Tif::Fok ) ).unwrap();
  assert!( short.trades.is_empty() && short.tif_dropped );
  assert_eq!( exchange.book(), &before );
  let seller = exchange.escrow().account( TAKER ).unwrap();
  assert_eq!( ( seller.asset.available(), seller.asset.reserved() ), ( qty( 100 ), Quantity::ZERO ) );

  let whole = place( &mut exchange, &mut producer, &mut consumer, order( TAKER, Side::Sell, "0", 4, Tif::Fok ) ).unwrap();
  let fills : Vec< _ > = whole.trades.iter().map( | trade | ( trade.price, trade.quantity ) ).collect();
  assert_eq!( fills, [ ( price( "2.00" ), qty( 2 ) ), ( price( "1.90" ), qty( 2 ) ) ] );
  let seller = exchange.escrow().account( TAKER ).unwrap();
  assert_eq!( ( seller.cash.available(), seller.asset.available(), seller.asset.reserved() ), ( money( "1007.80" ), qty( 96 ), Quantity::ZERO ) );
}
