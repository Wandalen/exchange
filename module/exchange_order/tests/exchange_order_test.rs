//! Test Matrix T01 — `Order`'s own fields, independent of `obligation()`'s
//! classification logic, which stays tested in `exchange_types`.
//!
//! No same-typed field pair exists on `Order` — every one of its seven fields
//! has a distinct type, so a struct-literal transposition is a compile error,
//! not a silent bug the way `exchange_spec`'s `base`/`quote` swap was. Nothing
//! here needs a mutation test the way that one did; see `tests/manual/readme.md`.

use exact_arith::{ Price, Quantity };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_side::Side;
use exchange_tif::Tif;

fn order() -> Order
{
  Order
  {
    id : OrderId( 1 ),
    instrument : InstrumentId( 2 ),
    account : AccountId( 3 ),
    side : Side::Buy,
    price : Price::parse( "1.25" ).unwrap(),
    quantity : Quantity::from_int( 4 ).unwrap(),
    tif : Tif::Ioc,
    client : None,
  }
}

/// Every field reads back exactly what was written — no default, no
/// cross-field bleed, and in particular the two fields the source design
/// added over the real struct (`instrument`, `tif`) land correctly.
#[ test ]
fn an_order_holds_every_field_distinctly()
{
  let submitted = order();

  assert_eq!( submitted.id, OrderId( 1 ) );
  assert_eq!( submitted.instrument, InstrumentId( 2 ) );
  assert_eq!( submitted.account, AccountId( 3 ) );
  assert_eq!( submitted.side, Side::Buy );
  assert_eq!( submitted.price, Price::parse( "1.25" ).unwrap() );
  assert_eq!( submitted.quantity, Quantity::from_int( 4 ).unwrap() );
  assert_eq!( submitted.tif, Tif::Ioc );
}

/// T01 — an order carries exact values, not primitives.
///
/// Asserted through arithmetic rather than through the type name: the point is
/// not that the field is called `Price`, it is that adding to it goes through
/// a checked operation that can refuse. A `f64` field would satisfy any
/// name-based check and fail this one.
#[ test ]
fn t01_an_order_holds_exact_values()
{
  let submitted = order();

  let doubled = submitted.price.checked_add( submitted.price );
  assert_eq!( doubled.unwrap(), Price::parse( "2.50" ).unwrap() );
}
