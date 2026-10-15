//! Test Matrix T01 — the vocabulary, and the two rules that live in it.
//!
//! Beyond T01's own row this file pins the notional's exactness and the
//! executed-price rule, because both are decisions this crate makes on behalf
//! of the whole family and neither is observable from the matching tests:
//! a rounded notional and an exact one produce identical trades and differ
//! only in a total nobody sums until an audit.
//!
//! `Order`'s own field-exactness test moved to `exchange_order/tests/`
//! alongside the type itself; what stays here exercises `obligation`/
//! `notional`, which stayed in this crate (see `src/lib.rs`'s "Extraction"
//! section). `every_side_has_exactly_one_opposite` moved to
//! `exchange_side/tests/` in Stage 1 and is not duplicated here, and
//! `a_trade_executes_at_the_makers_price` moved to `exchange_fill/tests/`
//! alongside `Trade` itself in Stage 5.

use exact_arith::{ Money, Quantity };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::{ Obligation, Order };
use exchange_side::Side;
use exchange_tif::Tif;
use exchange_types::{ TypeError, notional, obligation };

fn order( side : Side, price : &str, quantity : i64 ) -> Order
{
  Order
  {
    id : OrderId( 1 ),
    instrument : InstrumentId( 1 ),
    account : AccountId( 1 ),
    side,
    price : Money::parse( price ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif : Tif::Gtc,
  }
}

/// T01 — the two prices float cannot hold are held exactly here.
///
/// `0.1 + 0.2 == 0.3` is the canonical `f64` failure. If any part of the price
/// path were a float this would be false, and every other test in the
/// family would still pass.
#[ test ]
fn t01_the_price_path_is_not_floating_point()
{
  let tenth = Money::parse( "0.1" ).unwrap();
  let fifth = Money::parse( "0.2" ).unwrap();

  assert_eq!( tenth.checked_add( fifth ).unwrap(), Money::parse( "0.3" ).unwrap() );

  #[ allow( clippy::float_cmp ) ]
  let float_disagrees = 0.1_f64 + 0.2_f64 != 0.3_f64;
  assert!( float_disagrees, "the control this test exists to beat must actually be wrong" );
}

/// A buy commits currency at its own limit; a sell commits the asset itself.
#[ test ]
fn an_obligation_takes_the_shape_of_what_is_owed()
{
  let buy = obligation( &order( Side::Buy, "2.50", 4 ) ).unwrap();
  assert_eq!( buy, Obligation::Cash( Money::parse( "10" ).unwrap() ) );

  let sell = obligation( &order( Side::Sell, "2.50", 4 ) ).unwrap();
  assert_eq!( sell, Obligation::Asset( Quantity::from_int( 4 ).unwrap() ) );
}

/// A buy reserves at *its own* limit, never at a price it hopes to get.
///
/// Reserving less than the maximum is how a resting order becomes
/// un-executable: the book displays it, someone crosses it, and the funds are
/// not there.
#[ test ]
fn a_buy_reserves_its_worst_case_not_its_best()
{
  let generous = obligation( &order( Side::Buy, "3.00", 4 ) ).unwrap();
  let tight = obligation( &order( Side::Buy, "2.00", 4 ) ).unwrap();

  assert_eq!( generous, Obligation::Cash( Money::parse( "12" ).unwrap() ) );
  assert_eq!( tight, Obligation::Cash( Money::parse( "8" ).unwrap() ) );
}

/// A notional that would need rounding is refused, not rounded.
///
/// This is the whole reason `notional` returns a `Result`. Half a minor unit
/// per trade is invisible per trade and is exactly how value leaks: rounded
/// down it vanishes, rounded to nearest it does both and cancels only on
/// average.
#[ test ]
fn a_notional_needing_rounding_is_refused()
{
  // One minor unit of quantity — 0.000001 — at a price of 0.000001 is
  // 0.000000000001, twelve places, and the currency has six.
  let dust_price = Money::EPSILON;
  let dust_quantity = Quantity::EPSILON;

  assert_eq!( notional( dust_price, dust_quantity ), Err( TypeError::NotionalInexact ) );
}

/// A notional too large to express is refused too, and distinguishably.
#[ test ]
fn a_notional_over_the_ceiling_is_refused_by_name()
{
  let huge = Money::from_int( 1_000_000_000 ).unwrap();
  let many = Quantity::from_int( 1_000_000 ).unwrap();

  assert_eq!( notional( huge, many ), Err( TypeError::NotionalOutOfRange ) );
}

/// A notional over the ceiling by a second, narrower route — refused the same
/// way, but not on the same line.
///
/// The test above picks a quotient so large it does not even fit `Backing`'s
/// own width, so `Backing::try_from` is what refuses it. `notional` has a
/// second refusal a few lines later: a quotient that fits `Backing`
/// comfortably but still exceeds the currency's own declared
/// `CEILING_MINOR_UNITS`, which `Money::from_minor` refuses on its own
/// account, on its own `map_err`. Both happen to read `NotionalOutOfRange`
/// today, but only the first line had a test — a copy-paste that left the
/// second reading `NotionalInexact` instead would have shipped silently.
#[ test ]
fn a_notional_over_the_currencys_ceiling_but_within_backing_width_is_refused()
{
  // 1_000_000_000 whole units of price (minor 10^15) times 10 whole units of
  // quantity (minor 10^7) lands the quotient at 10^16: past the 9×10^15
  // currency ceiling, but nowhere near `Backing`'s (`i64`) own ~9.2×10^18 limit.
  let price = Money::from_int( 1_000_000_000 ).unwrap();
  let quantity = Quantity::from_int( 10 ).unwrap();

  assert_eq!( notional( price, quantity ), Err( TypeError::NotionalOutOfRange ) );
}

/// Exact notionals come back exact, at every scale the currency has.
#[ test ]
fn an_exact_notional_survives_the_round_trip()
{
  let cases =
  [
    ( "1.25", 4_i64, "5" ),
    ( "0.000001", 1_000_000, "1" ),
    ( "3", 7, "21" ),
    ( "0.5", 3, "1.5" ),
  ];

  for ( price, quantity, expected ) in cases
  {
    let got = notional( Money::parse( price ).unwrap(), Quantity::from_int( quantity ).unwrap() ).unwrap();
    assert_eq!( got, Money::parse( expected ).unwrap(), "{price} x {quantity}" );
  }
}

/// A zero-quantity order has no obligation, which is why the exchange refuses
/// it before escrow ever sees it.
#[ test ]
fn a_zero_quantity_order_commits_nothing()
{
  let empty = Order { quantity : Quantity::ZERO, ..order( Side::Buy, "2.50", 1 ) };
  assert_eq!( obligation( &empty ).unwrap(), Obligation::Cash( Money::ZERO ) );
}
