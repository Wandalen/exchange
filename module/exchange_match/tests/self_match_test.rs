//! Test Matrix T09–T13 — self-match prevention: no `Trade` may carry equal
//! self-match keys on both sides, each of the three configured policies
//! withdraws exactly its documented side, and a FOK meeting its own order is
//! still all or nothing.

use exact_arith::{ Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_match::{ SelfMatchCancellation, SelfMatchPolicy, CrossingStep, cross };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

fn order( id : u64, account : u64, side : Side, price : &str, quantity : i64 ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( account ),
    side,
    price : Price::parse( price ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif : Tif::Gtc,
    client : None,
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
/// coincidental id.
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
    vec![ SelfMatchCancellation { order : OrderId( 3 ), account : AccountId( 42 ), quantity : qty( 4 ), trades_before : 0 } ],
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
    vec![ SelfMatchCancellation { order : OrderId( 1 ), account : AccountId( 9 ), quantity : qty( 3 ), trades_before : 0 } ],
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
    vec![ SelfMatchCancellation { order : OrderId( 2 ), account : AccountId( 9 ), quantity : qty( 4 ), trades_before : 0 } ],
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
      SelfMatchCancellation { order : OrderId( 1 ), account : AccountId( 9 ), quantity : qty( 4 ), trades_before : 0 },
      SelfMatchCancellation { order : OrderId( 2 ), account : AccountId( 9 ), quantity : qty( 4 ), trades_before : 0 },
    ],
  );
  assert!( book.is_empty(), "the resting side is gone too" );
}

/// Cancel-resting for a buy taker whose own side is populated: its own asks,
/// across two price levels, are withdrawn in priority order with what was
/// left of each, the loop resumes into another account's ask behind them, and
/// every resting bid stays where it was.
///
/// T10 (`t10_cancel_resting_withdraws_the_resting_side_and_the_loop_resumes`)
/// and T12 (`t12_cancel_both_withdraws_both_sides`) leave the bid side empty,
/// so a withdrawal that disturbed the taker's own side would pass both.
#[ test ]
fn cancel_resting_for_a_buy_taker_leaves_the_bids_untouched()
{
  let mut book = Book::new();
  rest( &mut book, 1, 8, Side::Buy, "2.00", 5, 10 );
  rest( &mut book, 2, 8, Side::Buy, "1.90", 5, 20 );
  rest( &mut book, 3, 8, Side::Buy, "2.00", 5, 30 );
  rest( &mut book, 4, 9, Side::Sell, "2.50", 3, 40 );
  rest( &mut book, 5, 9, Side::Sell, "2.50", 2, 50 );
  rest( &mut book, 6, 9, Side::Sell, "2.60", 4, 60 );
  rest( &mut book, 7, 7, Side::Sell, "2.60", 5, 70 );

  // Another account takes 1 of order 4 first, so its withdrawal must report
  // what was left of it rather than what it was submitted with.
  let partial = cross( &mut book, &order( 8, 8, Side::Buy, "2.50", 1 ), SelfMatchPolicy::CancelResting ).unwrap();
  assert_eq!( partial.trades.len(), 1 );

  let crossing = cross( &mut book, &order( 9, 9, Side::Buy, "2.60", 5 ), SelfMatchPolicy::CancelResting ).unwrap();

  assert_eq!
  (
    crossing.cancelled,
    vec!
    [
      SelfMatchCancellation { order : OrderId( 4 ), account : AccountId( 9 ), quantity : qty( 2 ), trades_before : 0 },
      SelfMatchCancellation { order : OrderId( 5 ), account : AccountId( 9 ), quantity : qty( 2 ), trades_before : 0 },
      SelfMatchCancellation { order : OrderId( 6 ), account : AccountId( 9 ), quantity : qty( 4 ), trades_before : 0 },
    ],
  );
  assert_eq!( crossing.trades.len(), 1, "the loop resumed and filled against the other account's ask" );
  assert_eq!( crossing.trades[ 0 ].maker, OrderId( 7 ) );
  assert_eq!( crossing.trades[ 0 ].quantity, qty( 5 ) );
  assert!( crossing.is_complete() );

  let bids : Vec< ( OrderId, Quantity ) > = book.side( InstrumentId( 1 ), Side::Buy ).map( | resting | ( resting.order.id, resting.remaining ) ).collect();
  assert_eq!
  (
    bids,
    vec![ ( OrderId( 1 ), qty( 5 ) ), ( OrderId( 3 ), qty( 5 ) ), ( OrderId( 2 ), qty( 5 ) ) ],
    "every bid still rests, untouched and in the same order",
  );
  assert_eq!( book.side( InstrumentId( 1 ), Side::Sell ).count(), 0, "every ask is gone" );
}

/// Cancel-both for a buy taker whose own side is populated: the own ask and
/// the incoming remainder are withdrawn, and every resting bid stays where it
/// was.
#[ test ]
fn cancel_both_for_a_buy_taker_leaves_the_bids_untouched()
{
  let mut book = Book::new();
  rest( &mut book, 1, 8, Side::Buy, "2.00", 5, 10 );
  rest( &mut book, 2, 8, Side::Buy, "1.90", 5, 20 );
  rest( &mut book, 3, 9, Side::Sell, "2.50", 4, 30 );
  rest( &mut book, 4, 7, Side::Sell, "2.50", 5, 40 );

  let crossing = cross( &mut book, &order( 5, 9, Side::Buy, "2.50", 4 ), SelfMatchPolicy::CancelBoth ).unwrap();

  assert!( crossing.trades.is_empty() );
  assert_eq!
  (
    crossing.cancelled,
    vec!
    [
      SelfMatchCancellation { order : OrderId( 3 ), account : AccountId( 9 ), quantity : qty( 4 ), trades_before : 0 },
      SelfMatchCancellation { order : OrderId( 5 ), account : AccountId( 9 ), quantity : qty( 4 ), trades_before : 0 },
    ],
  );
  let ids = | side | book.side( InstrumentId( 1 ), side ).map( | resting | resting.order.id ).collect::< Vec< _ > >();
  assert_eq!( ids( Side::Buy ), vec![ OrderId( 1 ), OrderId( 2 ) ], "every bid still rests, in the same order" );
  assert_eq!( ids( Side::Sell ), vec![ OrderId( 4 ) ], "the other account's ask is untouched" );
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

/// T13 — a FOK that meets its own resting order before it can fill whole is
/// killed whole under cancel-incoming and cancel-both: no trade, nothing
/// withdrawn, the book as it was. Only the probe met the self-match.
#[ test ]
fn t13_a_self_crossing_fok_is_killed_whole_when_the_incoming_side_is_withdrawn()
{
  for policy in [ SelfMatchPolicy::CancelIncoming, SelfMatchPolicy::CancelBoth ]
  {
    let mut book = Book::new();
    rest( &mut book, 1, 8, Side::Sell, "2.50", 2, 10 );
    rest( &mut book, 2, 9, Side::Sell, "2.50", 2, 20 );
    let before = book.clone();

    let incoming = Order { tif : Tif::Fok, ..order( 3, 9, Side::Buy, "2.50", 4 ) };
    let crossing = cross( &mut book, &incoming, policy ).unwrap();

    assert!( crossing.trades.is_empty(), "{policy:?}: account 8's two units would have traded before the self-match" );
    assert!( crossing.cancelled.is_empty(), "{policy:?}: nothing left the real book" );
    assert_eq!( crossing.remaining, qty( 4 ) );
    assert_eq!( book, before, "{policy:?}" );
  }
}

/// T13 — under cancel-resting, a FOK withdraws its own resting order and
/// fills whole from the order behind it.
#[ test ]
fn t13_a_self_crossing_fok_under_cancel_resting_fills_past_its_own_order()
{
  let mut book = Book::new();
  rest( &mut book, 1, 9, Side::Sell, "2.50", 2, 10 );
  rest( &mut book, 2, 8, Side::Sell, "2.50", 4, 20 );

  let incoming = Order { tif : Tif::Fok, ..order( 3, 9, Side::Buy, "2.50", 4 ) };
  let crossing = cross( &mut book, &incoming, SelfMatchPolicy::CancelResting ).unwrap();

  assert_eq!
  (
    crossing.cancelled,
    vec![ SelfMatchCancellation { order : OrderId( 1 ), account : AccountId( 9 ), quantity : qty( 2 ), trades_before : 0 } ],
  );
  assert_eq!( crossing.trades.len(), 1 );
  assert_eq!( crossing.trades[ 0 ].maker, OrderId( 2 ) );
  assert!( crossing.is_complete() );
  assert!( book.is_empty() );
}

/// T13 — under cancel-resting, a FOK still short once its own order is
/// passed over leaves that order resting: only the probe withdrew it.
#[ test ]
fn t13_a_self_crossing_fok_that_cannot_fill_leaves_its_own_order_resting()
{
  let mut book = Book::new();
  rest( &mut book, 1, 9, Side::Sell, "2.50", 2, 10 );
  rest( &mut book, 2, 8, Side::Sell, "2.50", 3, 20 );
  let before = book.clone();

  let incoming = Order { tif : Tif::Fok, ..order( 3, 9, Side::Buy, "2.50", 4 ) };
  let crossing = cross( &mut book, &incoming, SelfMatchPolicy::CancelResting ).unwrap();

  assert!( crossing.trades.is_empty() );
  assert!( crossing.cancelled.is_empty() );
  assert_eq!( crossing.remaining, qty( 4 ) );
  assert_eq!( book, before, "order 1 still rests" );
}

/// A withdrawal records where it fell among the trades, and `steps` puts it
/// back there: a cancel-resting taker trades, passes over its own order, and
/// trades again behind it.
#[ test ]
fn steps_keep_a_withdrawal_between_the_trades_around_it()
{
  let mut book = Book::new();
  rest( &mut book, 1, 8, Side::Sell, "2.50", 2, 10 );
  rest( &mut book, 2, 9, Side::Sell, "2.50", 2, 20 );
  rest( &mut book, 3, 7, Side::Sell, "2.60", 4, 30 );

  let crossing = cross( &mut book, &order( 4, 9, Side::Buy, "2.60", 5 ), SelfMatchPolicy::CancelResting ).unwrap();

  assert_eq!( crossing.cancelled[ 0 ].trades_before, 1, "order 1 traded before order 2 was withdrawn" );
  let steps : Vec< String > = crossing.steps().map( | step | match step
  {
    CrossingStep::Trade( trade ) => format!( "trade {}", trade.maker.0 ),
    CrossingStep::Cancelled( withdrawn ) => format!( "cancel {}", withdrawn.order.0 ),
  } ).collect();
  assert_eq!( steps, vec![ "trade 1", "cancel 2", "trade 3" ] );
}
