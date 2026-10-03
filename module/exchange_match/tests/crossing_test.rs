//! Test Matrix T05–T07 — the three outcomes of crossing, and the price it
//! executes at.
//!
//! T07 is the row that matters most and the easiest to leave out. An engine
//! that fills unconditionally satisfies T05 and T06 completely; only the
//! no-cross case distinguishes a matching engine from a machine that pairs
//! whatever it is handed.

use exact_arith::{ Money, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::InstrumentId;
use exchange_match::{ Crossing, MatchError, SelfMatchPolicy, cross as cross_with_policy };
use exchange_tif::Tif;
use exchange_types::{ AccountId, Order, OrderId, Sequence, Side };

/// This file's own `order()` helper gives every order a unique account
/// (`account : AccountId( id )`), so no call below can ever self-match — the
/// policy argument is structurally inert here. Fixed to one arbitrary choice
/// so T05–T07's own call sites stay free of a decision they have no stake in;
/// self-match policy itself is `tests/self_match_test.rs`'s own T09–T12.
fn cross( book : &mut Book, incoming : &Order ) -> Result< Crossing, MatchError >
{
  cross_with_policy( book, incoming, SelfMatchPolicy::CancelResting )
}

fn order( id : u64, side : Side, price : &str, quantity : i64 ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( id ),
    side,
    price : Money::parse( price ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif : Tif::Gtc,
  }
}

fn rest( book : &mut Book, id : u64, side : Side, price : &str, quantity : i64, arrival : u64 )
{
  let order = order( id, side, price, quantity );
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

/// T05 — an exact cross: one trade, both orders gone.
#[ test ]
fn t05_an_exact_cross_fills_completely()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "2.50", 4, 10 );

  let crossing = cross( &mut book, &order( 2, Side::Buy, "2.50", 4 ) ).unwrap();

  assert_eq!( crossing.trades.len(), 1 );
  assert_eq!( crossing.trades[ 0 ].quantity, qty( 4 ) );
  assert_eq!( crossing.remaining, Quantity::ZERO );
  assert!( crossing.is_complete() );
  assert!( book.is_empty(), "the resting order was consumed entirely and left" );
}

/// T06 — a taker larger than the book: partial fill, remainder returned.
///
/// The remainder is asserted, not only the fill size. An engine that filled 4
/// and reported a remainder of 0 would satisfy a fill-only check and quietly
/// destroy 6 units of a customer's order.
#[ test ]
fn t06_a_taker_larger_than_the_book_fills_partially()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "2.50", 4, 10 );

  let crossing = cross( &mut book, &order( 2, Side::Buy, "2.50", 10 ) ).unwrap();

  assert_eq!( crossing.trades.len(), 1 );
  assert_eq!( crossing.filled().unwrap(), qty( 4 ), "it took everything available" );
  assert_eq!( crossing.remaining, qty( 6 ), "and reported exactly what it could not get" );
  assert!( !crossing.is_complete() );
  assert!( book.is_empty() );
}

/// T06 — the parts sum to the whole, across several resting orders.
///
/// The conservation clause on splitting: filled + remaining is the submitted
/// quantity, with no slack anywhere.
#[ test ]
fn t06_the_parts_sum_to_the_submitted_quantity()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "2.50", 3, 10 );
  rest( &mut book, 2, Side::Sell, "2.50", 3, 20 );
  rest( &mut book, 3, Side::Sell, "9.00", 3, 30 );

  let incoming = order( 4, Side::Buy, "2.50", 10 );
  let crossing = cross( &mut book, &incoming ).unwrap();

  assert_eq!( crossing.trades.len(), 2, "the third ask is priced beyond the bid" );
  let total = crossing.filled().unwrap().checked_add( crossing.remaining ).unwrap();
  assert_eq!( total, incoming.quantity, "nothing was created or lost in the split" );
}

/// T07 — priced through no liquidity: zero trades, everything remaining.
///
/// This is the control arm's case, asserted here as well as in the lane
/// because the lane proves the engine *can* decline and this proves it
/// declines for the right reason — the book still holds its ask afterwards.
#[ test ]
fn t07_an_order_priced_through_no_liquidity_does_not_cross()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "2.50", 4, 10 );

  let crossing = cross( &mut book, &order( 2, Side::Buy, "2.49", 4 ) ).unwrap();

  assert_eq!( crossing.trades.len(), 0, "one minor unit short is short" );
  assert_eq!( crossing.remaining, qty( 4 ), "the whole quantity is untouched" );
  assert_eq!( book.best( InstrumentId( 1 ), Side::Sell ).unwrap().remaining, qty( 4 ), "and so is the ask" );
}

/// T07 — an empty book crosses nothing, and says so rather than failing.
#[ test ]
fn t07_an_empty_book_crosses_nothing()
{
  let mut book = Book::new();

  let crossing = cross( &mut book, &order( 1, Side::Buy, "2.50", 4 ) ).unwrap();

  assert_eq!( crossing.trades.len(), 0 );
  assert_eq!( crossing.remaining, qty( 4 ) );
}

/// The same three outcomes from the other side of the book.
///
/// A seller crosses bids *at or above* its floor — the mirror of the buyer's
/// rule, and the place a copy-pasted comparison would be caught.
#[ test ]
fn a_seller_crosses_bids_at_or_above_its_floor()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Buy, "2.50", 4, 10 );

  let takes = cross( &mut book, &order( 2, Side::Sell, "2.50", 4 ) ).unwrap();
  assert_eq!( takes.trades.len(), 1, "a sell at the bid crosses it" );

  let mut book = Book::new();
  rest( &mut book, 1, Side::Buy, "2.50", 4, 10 );
  let declines = cross( &mut book, &order( 2, Side::Sell, "2.51", 4 ) ).unwrap();
  assert_eq!( declines.trades.len(), 0, "a sell above the bid does not" );
}

/// Equality crosses. An order at exactly the other side's price is a match.
#[ test ]
fn an_order_at_exactly_the_other_sides_price_crosses()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "2.50", 4, 10 );

  let crossing = cross( &mut book, &order( 2, Side::Buy, "2.50", 4 ) ).unwrap();
  assert_eq!( crossing.trades.len(), 1, "at the price is not through the price" );
}

/// The loop takes from the front, so it consumes in the book's published order.
///
/// Asserted by the ids in the trades rather than by the book's remains,
/// because those ids are what a settlement path uses to decide who gets paid.
#[ test ]
fn the_loop_consumes_in_the_books_published_order()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "2.00", 2, 30 );
  rest( &mut book, 2, Side::Sell, "1.00", 2, 20 );
  rest( &mut book, 3, Side::Sell, "2.00", 2, 10 );

  let crossing = cross( &mut book, &order( 4, Side::Buy, "3.00", 6 ) ).unwrap();

  let makers : Vec< u64 > = crossing.trades.iter().map( | trade | trade.maker.0 ).collect();
  assert_eq!( makers, vec![ 2, 3, 1 ], "cheapest first, then earliest arrival within the price" );
}

/// Each trade executes at its own maker's price, not at one blended price.
///
/// A taker sweeping two price levels pays each level's own price. Charging the
/// worst level's price for all of it, or the best, are both plausible-looking
/// implementations that this catches.
#[ test ]
fn each_trade_executes_at_its_own_makers_price()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "1.00", 2, 10 );
  rest( &mut book, 2, Side::Sell, "2.00", 2, 20 );

  let crossing = cross( &mut book, &order( 3, Side::Buy, "3.00", 4 ) ).unwrap();

  let prices : Vec< Money > = crossing.trades.iter().map( | trade | trade.price ).collect();
  assert_eq!( prices, vec![ Money::parse( "1" ).unwrap(), Money::parse( "2" ).unwrap() ] );
}

/// Both sides travel in the trade, and the aggressor is the incoming order.
///
/// The maker/taker classification is a property of the record rather than
/// something each consumer re-derives — and re-derives differently.
#[ test ]
fn a_trade_names_both_of_its_sides()
{
  let mut book = Book::new();
  rest( &mut book, 1, Side::Sell, "2.50", 4, 10 );

  let incoming = order( 2, Side::Buy, "2.50", 4 );
  let crossing = cross( &mut book, &incoming ).unwrap();
  let trade = crossing.trades[ 0 ];

  assert_eq!( trade.taker, incoming.id );
  assert_eq!( trade.taker_account, incoming.account );
  assert_eq!( trade.maker, OrderId( 1 ) );
  assert_eq!( trade.maker_account, AccountId( 1 ) );
}

/// Crossing is a pure function of the book and the incoming order.
///
/// The determinism clause, tested the only way it can be from inside one
/// process: the same inputs twice, and the outputs compared in full. A rule
/// that consulted a clock, a hash iteration order, or an address would have to
/// be very unlucky to differ across two adjacent runs — so this is a weak
/// check on its own, which is why the crate forbids those inputs by
/// construction rather than relying on it.
#[ test ]
fn crossing_the_same_book_twice_gives_the_same_trades()
{
  let build = ||
  {
    let mut book = Book::new();
    rest( &mut book, 1, Side::Sell, "2.00", 2, 30 );
    rest( &mut book, 2, Side::Sell, "1.00", 2, 20 );
    rest( &mut book, 3, Side::Sell, "2.00", 2, 10 );
    book
  };

  let first = cross( &mut build(), &order( 9, Side::Buy, "3.00", 5 ) ).unwrap();
  let second = cross( &mut build(), &order( 9, Side::Buy, "3.00", 5 ) ).unwrap();

  assert_eq!( first, second );
}
