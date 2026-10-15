//! Test Matrix — time-in-force: IOC's no-op parity with GTC, and FOK's
//! dry-run-then-commit split.
//!
//! IOC has no outcome of its own to test beyond "behaves exactly like GTC,
//! here" — the module doc's own argument for why `cross` reads `tif` in
//! exactly one place. FOK is the real behavioural branch: book-untouched on
//! a partial, replayed-for-real on a full fill.

use exact_kind::{ Money, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_match::{ Crossing, MatchError, SelfMatchPolicy, cross as cross_with_policy };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

fn cross( book : &mut Book, incoming : &Order ) -> Result< Crossing, MatchError >
{
  cross_with_policy( book, incoming, SelfMatchPolicy::CancelResting )
}

fn order( id : u64, side : Side, price : &str, quantity : i64, tif : Tif ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( id ),
    side,
    price : Money::parse( price ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif,
  }
}

fn rest( book : &mut Book, id : u64, side : Side, price : &str, quantity : i64, arrival : u64 )
{
  let order = order( id, side, price, quantity, Tif::Gtc );
  assert!
  (
    book.insert( Resting { order, remaining : order.quantity, arrival : Sequence( arrival ) } ),
    "fresh id in this test, must not already rest",
  );
}

fn qty( whole : i64 ) -> Quantity
{
  Quantity::from_int( whole ).unwrap()
}

/// IOC fills what it can, same as GTC — the only difference is a remainder a
/// caller must not rest, which `cross` itself has never inserted for any TIF
/// (see the module doc's "Time-in-force" section).
#[ test ]
fn ioc_fills_partially_and_reports_the_same_remainder_gtc_would()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "2.50", 4, 10 );

  let crossing = cross( &mut book, &order( 2, Side::Buy, "2.50", 10, Tif::Ioc ) ).unwrap();

  assert_eq!( crossing.filled().unwrap(), qty( 4 ) );
  assert_eq!( crossing.remaining, qty( 6 ) );
  assert!( book.is_empty(), "the resting ask was fully taken" );
}

/// IOC on an empty book crosses nothing — same shape as GTC's no-cross.
#[ test ]
fn ioc_against_no_liquidity_crosses_nothing()
{
  let mut book = Book::new();

  let crossing = cross( &mut book, &order( 1, Side::Buy, "2.50", 4, Tif::Ioc ) ).unwrap();

  assert_eq!( crossing.trades.len(), 0 );
  assert_eq!( crossing.remaining, qty( 4 ) );
}

/// A FOK that can be filled completely trades for real, exactly like GTC —
/// the probe's clone is not the only place the fill happens.
#[ test ]
fn fok_that_fits_entirely_fills_and_touches_the_real_book()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "2.50", 4, 10 );

  let crossing = cross( &mut book, &order( 2, Side::Buy, "2.50", 4, Tif::Fok ) ).unwrap();

  assert_eq!( crossing.filled().unwrap(), qty( 4 ) );
  assert!( crossing.is_complete() );
  assert!( book.is_empty(), "the real book reflects the fill, not just the probe" );
}

/// A FOK that cannot fill completely rejects without touching the book at
/// all — the resting order is untouched, not partially consumed then undone.
#[ test ]
fn fok_that_cannot_fill_entirely_leaves_the_book_untouched()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "2.50", 4, 10 );

  let crossing = cross( &mut book, &order( 2, Side::Buy, "2.50", 10, Tif::Fok ) ).unwrap();

  assert_eq!( crossing.trades.len(), 0, "an all-or-nothing order that can't get all gets nothing" );
  assert_eq!( crossing.remaining, qty( 10 ), "the full original quantity, not a partial's leftover" );
  assert_eq!
  (
    book.best( InstrumentId( 1 ), Side::Sell ).unwrap().remaining,
    qty( 4 ),
    "the resting ask is exactly as it was — not consumed then restored",
  );
}

/// FOK against an empty book is just the empty-book no-cross case.
#[ test ]
fn fok_against_no_liquidity_crosses_nothing()
{
  let mut book = Book::new();

  let crossing = cross( &mut book, &order( 1, Side::Buy, "4", 4, Tif::Fok ) ).unwrap();

  assert_eq!( crossing.trades.len(), 0 );
  assert_eq!( crossing.remaining, qty( 4 ) );
}

/// A FOK short by a second price level still rejects the whole order — a
/// fill that drains the cheaper level but comes up short overall is a full
/// reject, not a partial one, and the cheaper level must come back untouched.
#[ test ]
fn fok_short_by_a_second_level_rejects_the_whole_order_and_restores_the_first()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "1.00", 2, 10 );
  rest( &mut book, 2, Side::Sell, "2.00", 1, 20 );

  let crossing = cross( &mut book, &order( 3, Side::Buy, "3.00", 4, Tif::Fok ) ).unwrap();

  assert_eq!( crossing.trades.len(), 0, "only 3 of 4 units are reachable at any acceptable price" );
  assert_eq!( crossing.remaining, qty( 4 ) );
  assert_eq!( book.best( InstrumentId( 1 ), Side::Sell ).unwrap().remaining, qty( 2 ), "cheaper level untouched" );
}

/// A FOK that exactly exhausts two price levels still fills for real.
#[ test ]
fn fok_that_exactly_exhausts_two_levels_fills_both()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "1.00", 2, 10 );
  rest( &mut book, 2, Side::Sell, "2.00", 2, 20 );

  let crossing = cross( &mut book, &order( 3, Side::Buy, "3.00", 4, Tif::Fok ) ).unwrap();

  assert_eq!( crossing.trades.len(), 2 );
  assert!( crossing.is_complete() );
  assert!( book.is_empty() );
}
