//! Test Matrix T01 — the executed-price rule, and `Trade`'s own shape.
//!
//! Migrated from `exchange_types/tests/order_types_test.rs` along with
//! `Trade` itself — see that file's own module doc for the full migration
//! note.

use exact_arith::{ Price, Quantity };
use exchange_fill::{ EventKind, RejectReason, Trade };
use exchange_id::{ AccountId, OrderId };
use exchange_side::Side;

/// The executed-price rule: the maker's price wins, whatever the taker offered.
///
/// Pinned as its own test because it is a decision the family's design
/// documents leave open, and a silent change of convention here would move
/// money without changing a single quantity — the spread would simply start
/// accruing to whoever arrived second.
#[ test ]
fn a_trade_executes_at_the_makers_price()
{
  let maker = Price::parse( "2.50" ).unwrap();
  let generous_taker = Price::parse( "3.00" ).unwrap();

  assert_eq!( Trade::executed_price( maker, generous_taker ), maker );
  assert_ne!( Trade::executed_price( maker, generous_taker ), generous_taker );
}

/// Every field on a constructed `Trade` reads back exactly what was written —
/// in particular `taker_side`, the field added over the real struct.
#[ test ]
fn a_trade_holds_every_field_distinctly()
{
  let trade = Trade
  {
    taker : OrderId( 1 ),
    taker_account : AccountId( 1 ),
    taker_side : Side::Buy,
    maker : OrderId( 2 ),
    maker_account : AccountId( 2 ),
    price : Price::parse( "2.50" ).unwrap(),
    quantity : Quantity::from_int( 4 ).unwrap(),
  };

  assert_eq!( trade.taker, OrderId( 1 ) );
  assert_eq!( trade.taker_account, AccountId( 1 ) );
  assert_eq!( trade.taker_side, Side::Buy );
  assert_eq!( trade.maker, OrderId( 2 ) );
  assert_eq!( trade.maker_account, AccountId( 2 ) );
  assert_eq!( trade.price, Price::parse( "2.50" ).unwrap() );
  assert_eq!( trade.quantity, Quantity::from_int( 4 ).unwrap() );
}

/// Phase P16 — a `Fill` (here, the `EventKind::Trade` payload) names both the
/// maker and the taker explicitly, not just the aggressor.
#[ test ]
fn p16_fill_names_maker_and_taker()
{
  let trade = Trade
  {
    taker : OrderId( 1 ),
    taker_account : AccountId( 1 ),
    taker_side : Side::Buy,
    maker : OrderId( 2 ),
    maker_account : AccountId( 2 ),
    price : Price::parse( "2.50" ).unwrap(),
    quantity : Quantity::from_int( 4 ).unwrap(),
  };
  let event = EventKind::Trade( trade );

  let EventKind::Trade( named ) = event
  else
  {
    panic!( "constructed an EventKind::Trade, got something else back" )
  };
  assert_eq!( named.maker, OrderId( 2 ) );
  assert_eq!( named.taker, OrderId( 1 ) );
}

/// `RejectReason` stays a plain, exhaustively-matchable closed set across the
/// move — a smoke check that the migration did not silently widen it.
#[ test ]
fn reject_reason_is_still_exhaustively_matchable()
{
  let reason = RejectReason::ZeroQuantity;
  let name = match reason
  {
    RejectReason::ZeroQuantity => "zero_quantity",
    RejectReason::NegativePrice => "negative_price",
    RejectReason::UnknownAccount => "unknown_account",
    RejectReason::InsufficientFunds => "insufficient_funds",
    RejectReason::ObligationUnrepresentable => "obligation_unrepresentable",
    RejectReason::ReservationUnrepresentable => "reservation_unrepresentable",
    RejectReason::Halted => "halted",
    RejectReason::RestsFull => "rests_full",
    RejectReason::LevelsFull => "levels_full",
    RejectReason::PostOnlyWouldTake => "post_only_would_take",
    RejectReason::DuplicateClientId => "duplicate_client_id",
    RejectReason::AccountFull => "account_full",
    RejectReason::PriceOffTick => "price_off_tick",
    RejectReason::QuantityOffLot => "quantity_off_lot",
  };
  assert_eq!( name, "zero_quantity" );
}
