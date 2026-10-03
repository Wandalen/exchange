//! Test Matrix T09–T12 — self-match prevention: no `Trade` may carry equal
//! self-match keys on both sides, and each of the three configured policies
//! withdraws exactly its documented side.
//!
//! T13 (a self-crossing FOK cancelled whole) is not testable yet. [`Order`]
//! carries a real `tif` field now, but `cross()` does not consult it —
//! matching stays GTC-only until Stage 6 of the family's own refactor plan
//! wires TIF-aware behavior through the crossing loop. Named here as open
//! rather than silently dropped or faked with behavior the crate does not
//! have.

use exact_arith::{ Money, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::InstrumentId;
use exchange_match::{ SelfMatchCancellation, SelfMatchPolicy, cross };
use exchange_tif::Tif;
use exchange_types::{ AccountId, Order, OrderId, Sequence, Side };

fn order( id : u64, account : u64, side : Side, price : &str, quantity : i64 ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( account ),
    side,
    price : Money::parse( price ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif : Tif::Gtc,
  }
}

fn rest( book : &mut Book, id : u64, account : u64, side : Side, price : &str, quantity : i64, arrival : u64 )
{
  let order = order( id, account, side, price, quantity );
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

/// T09 — across a flow mixing three accounts, no `Trade` ever carries equal
/// self-match keys on both sides, and the check does not block the
/// legitimate cross-account trading alongside it.
///
/// Two distinct colliding accounts (7 and 42, not just one hardcoded pair),
/// so the assertion rests on key equality itself rather than on one
/// coincidental id — the same concern
/// `substrate/exchange/exchange_core/task/unverified/082_implement_exchange_core.md`'s
/// AF3 names.
#[ test ]
fn t09_no_trade_in_a_mixed_account_flow_carries_equal_self_match_keys()
{
  let mut book = Book::new();
  rest( &mut book, 1, 7, Side::Sell, "2.50", 4, 10 );
  rest( &mut book, 2, 8, Side::Sell, "2.50", 4, 20 );
  rest( &mut book, 3, 42, Side::Sell, "2.50", 4, 30 );

  // Account 7's incoming order meets its own resting order first in the
  // book — skipped — then legitimately fills against account 8's, leaving
  // account 42's resting order untouched.
  let first = cross( &mut book, &order( 4, 7, Side::Buy, "2.50", 4 ), SelfMatchPolicy::CancelResting ).unwrap();
  assert!( first.trades.iter().all( | trade | trade.taker_account != trade.maker_account ) );
  assert_eq!( first.trades.len(), 1, "the legitimate account-8 fill must not have been blocked too" );
  assert_eq!( first.trades[ 0 ].maker_account, AccountId( 8 ) );

  // Account 42's incoming order now meets its own resting order, with
  // nothing legitimate left behind it.
  let second = cross( &mut book, &order( 5, 42, Side::Buy, "2.50", 4 ), SelfMatchPolicy::CancelResting ).unwrap();
  assert!( second.trades.is_empty() );
  assert_eq!
  (
    second.cancelled,
    vec![ SelfMatchCancellation { order : OrderId( 3 ), account : AccountId( 42 ), quantity : qty( 4 ) } ],
  );
}

/// T10 — Cancel-resting: the resting order is withdrawn with cause
/// self-match, and the incoming order's match loop resumes against whatever
/// is behind it.
#[ test ]
fn t10_cancel_resting_withdraws_the_resting_side_and_the_loop_resumes()
{
  let mut book = Book::new();
  rest( &mut book, 1, 9, Side::Sell, "2.50", 3, 10 );
  rest( &mut book, 2, 8, Side::Sell, "2.50", 5, 20 );

  let incoming = order( 3, 9, Side::Buy, "2.50", 5 );
  let crossing = cross( &mut book, &incoming, SelfMatchPolicy::CancelResting ).unwrap();

  assert_eq!
  (
    crossing.cancelled,
    vec![ SelfMatchCancellation { order : OrderId( 1 ), account : AccountId( 9 ), quantity : qty( 3 ) } ],
  );
  assert_eq!( crossing.trades.len(), 1, "the loop resumed and filled against the other resting order" );
  assert_eq!( crossing.trades[ 0 ].maker, OrderId( 2 ) );
  assert_eq!( crossing.trades[ 0 ].quantity, qty( 5 ) );
  assert!( crossing.is_complete() );
  assert!( book.cancel( InstrumentId( 1 ), OrderId( 1 ) ).is_none(), "the self-matched resting order is already gone from the book" );
}

/// T11 — Cancel-incoming: the incoming order's remainder is withdrawn with
/// cause self-match, and the resting order survives completely untouched.
#[ test ]
fn t11_cancel_incoming_withdraws_the_incoming_side_and_resting_survives()
{
  let mut book = Book::new();
  rest( &mut book, 1, 9, Side::Sell, "2.50", 4, 10 );

  let incoming = order( 2, 9, Side::Buy, "2.50", 4 );
  let crossing = cross( &mut book, &incoming, SelfMatchPolicy::CancelIncoming ).unwrap();

  assert!( crossing.trades.is_empty() );
  assert_eq!
  (
    crossing.cancelled,
    vec![ SelfMatchCancellation { order : OrderId( 2 ), account : AccountId( 9 ), quantity : qty( 4 ) } ],
  );
  let resting = book.best( InstrumentId( 1 ), Side::Sell ).unwrap();
  assert_eq!( resting.order.id, OrderId( 1 ) );
  assert_eq!( resting.remaining, qty( 4 ), "untouched — not even partially consumed" );
}

/// T12 — Cancel-both: both remainders are withdrawn with cause self-match.
#[ test ]
fn t12_cancel_both_withdraws_both_sides()
{
  let mut book = Book::new();
  rest( &mut book, 1, 9, Side::Sell, "2.50", 4, 10 );

  let incoming = order( 2, 9, Side::Buy, "2.50", 4 );
  let crossing = cross( &mut book, &incoming, SelfMatchPolicy::CancelBoth ).unwrap();

  assert!( crossing.trades.is_empty() );
  assert_eq!
  (
    crossing.cancelled,
    vec!
    [
      SelfMatchCancellation { order : OrderId( 1 ), account : AccountId( 9 ), quantity : qty( 4 ) },
      SelfMatchCancellation { order : OrderId( 2 ), account : AccountId( 9 ), quantity : qty( 4 ) },
    ],
  );
  assert!( book.is_empty(), "the resting side is gone too" );
}

/// C4 — the self-match key comparison runs before a candidate fill is
/// committed, not after. Proven by the case where the incoming order is
/// larger than the self-matching resting quantity: a comparison applied only
/// after computing a fill would still generate a partial trade for the part
/// that "fits" before cancelling the rest. None is generated here.
#[ test ]
fn a_self_match_is_caught_even_when_the_incoming_order_is_larger()
{
  let mut book = Book::new();
  rest( &mut book, 1, 9, Side::Sell, "2.50", 3, 10 );

  let incoming = order( 2, 9, Side::Buy, "2.50", 10 );
  let crossing = cross( &mut book, &incoming, SelfMatchPolicy::CancelIncoming ).unwrap();

  assert!( crossing.trades.is_empty(), "no partial fill against the self-matching quantity either" );
  assert_eq!( crossing.remaining, qty( 10 ), "the whole incoming quantity, not just the crossed part" );
}

/// A resting order priced through — never actually a crossing candidate —
/// is not treated as a self-match even when it shares the incoming order's
/// account. The check applies only to a pair about to trade.
#[ test ]
fn a_same_account_resting_order_that_does_not_cross_is_not_a_self_match()
{
  let mut book = Book::new();
  rest( &mut book, 1, 9, Side::Sell, "2.60", 4, 10 );

  let incoming = order( 2, 9, Side::Buy, "2.50", 4 );
  let crossing = cross( &mut book, &incoming, SelfMatchPolicy::CancelBoth ).unwrap();

  assert!( crossing.cancelled.is_empty(), "priced through — never a crossing candidate at all" );
  assert!( crossing.trades.is_empty() );
  assert_eq!( crossing.remaining, qty( 4 ) );
  assert_eq!( book.best( InstrumentId( 1 ), Side::Sell ).unwrap().remaining, qty( 4 ), "the resting order rests, untouched, either way" );
}
