//! Test Matrix — the three non-matching ways an order moves on the book:
//! `rest_place`, `rest_cancel`, `rest_replace`.
//!
//! `rest_place`/`rest_cancel` are direct call-throughs to `Book::insert`/
//! `Book::cancel`, already exhaustively covered by `exchange_book`'s own
//! test suite — these just confirm the wrapper forwards faithfully, not a
//! second copy of `Book`'s own matrix. `rest_replace` is this crate's one
//! genuinely new operation and gets the deeper coverage, including the
//! rollback path a plain cancel-then-insert can't offer atomically.

use exact_arith::{ Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_rest::{ RestReplaceError, rest_cancel, rest_place, rest_replace };
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

fn order( id : u64, side : Side, price : &str, quantity : i64 ) -> Order
{
  Order
  {
    id : OrderId( id ),
    instrument : InstrumentId( 1 ),
    account : AccountId( id ),
    side,
    price : Price::parse( price ).unwrap(),
    quantity : Quantity::from_int( quantity ).unwrap(),
    tif : Tif::Gtc,
    client : None,
  }
}

fn resting( id : u64, side : Side, price : &str, quantity : i64, arrival : u64 ) -> Resting
{
  let order = order( id, side, price, quantity );
  Resting { order, remaining : order.quantity, arrival : Sequence( arrival ) }
}

fn qty( whole : i64 ) -> Quantity
{
  Quantity::from_int( whole ).unwrap()
}

// --- rest_place -------------------------------------------------------

#[ test ]
fn rest_place_seats_a_fresh_order()
{
  let mut book = Book::new();

  assert!( rest_place( &mut book, resting( 1, Side::Sell, "2.50", 4, 10 ) ) );
  assert_eq!( book.best( InstrumentId( 1 ), Side::Sell ).unwrap().order.id, OrderId( 1 ) );
}

#[ test ]
fn rest_place_refuses_a_duplicate_id()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Sell, "2.50", 4, 10 ) ) );

  assert!( !rest_place( &mut book, resting( 1, Side::Sell, "9.00", 1, 20 ) ), "id 1 already rests" );
  assert_eq!( book.len(), 1, "the duplicate attempt seated nothing" );
}

// --- rest_cancel --------------------------------------------------------

#[ test ]
fn rest_cancel_withdraws_a_resting_order()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Sell, "2.50", 4, 10 ) ) );

  let removed = rest_cancel( &mut book, InstrumentId( 1 ), OrderId( 1 ) ).unwrap();

  assert_eq!( removed.order.id, OrderId( 1 ) );
  assert!( book.is_empty() );
}

#[ test ]
fn rest_cancel_on_a_missing_id_returns_none()
{
  let mut book = Book::new();

  assert!( rest_cancel( &mut book, InstrumentId( 1 ), OrderId( 404 ) ).is_none() );
}

// --- rest_replace: success ----------------------------------------------

/// The ordinary case this crate exists for: price/quantity amended in one
/// atomic step, old gone, new resting in its place.
#[ test ]
fn rest_replace_swaps_price_and_quantity_atomically()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Sell, "2.50", 4, 10 ) ) );

  let old = rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), resting( 1, Side::Sell, "2.60", 6, 20 ) ).unwrap();

  assert_eq!( old.order.price, Price::parse( "2.50" ).unwrap(), "the returned value is the pre-replace order" );
  let now = book.best( InstrumentId( 1 ), Side::Sell ).unwrap();
  assert_eq!( now.order.price, Price::parse( "2.60" ).unwrap() );
  assert_eq!( now.remaining, qty( 6 ) );
  assert_eq!( book.len(), 1, "exactly one order rests — not the old plus the new" );
}

/// A replacement that moves an order to a worse price loses queue priority
/// to whatever already rested at the better price — the same rule any
/// fresh `insert` follows, since `rest_replace` adds no priority rule of
/// its own.
#[ test ]
fn rest_replace_at_a_new_price_loses_its_old_queue_position()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Sell, "1.00", 4, 10 ) ) );
  assert!( rest_place( &mut book, resting( 2, Side::Sell, "1.00", 4, 20 ) ) );

  rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), resting( 1, Side::Sell, "1.01", 4, 30 ) ).unwrap();

  assert_eq!( book.best( InstrumentId( 1 ), Side::Sell ).unwrap().order.id, OrderId( 2 ), "order 2's price now ranks ahead" );
}

// --- rest_replace: Missing -----------------------------------------------

#[ test ]
fn rest_replace_on_a_missing_id_is_refused_and_touches_nothing()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Sell, "2.50", 4, 10 ) ) );

  let error = rest_replace( &mut book, InstrumentId( 1 ), OrderId( 404 ), resting( 404, Side::Sell, "9.00", 1, 20 ) ).unwrap_err();

  assert_eq!( error, RestReplaceError::Missing );
  assert_eq!( book.len(), 1, "the unrelated order 1 is untouched" );
}

// --- rest_replace: Refused, with rollback --------------------------------

/// The rollback path this crate exists to provide: a replacement that
/// collides with a third order is refused, and the original comes back
/// exactly as it was rather than staying cancelled.
#[ test ]
fn rest_replace_refused_by_a_colliding_id_restores_the_original()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Sell, "2.50", 4, 10 ) ) );
  assert!( rest_place( &mut book, resting( 2, Side::Sell, "3.00", 1, 20 ) ) );

  // Order 1 tries to replace itself with a `Resting` that happens to carry
  // order 2's id — refused by `Book::insert`'s duplicate-id check.
  let error = rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), resting( 2, Side::Sell, "2.60", 6, 30 ) ).unwrap_err();

  assert_eq!( error, RestReplaceError::Refused );
  assert_eq!( book.len(), 2, "neither order was lost" );
  let restored = book.best( InstrumentId( 1 ), Side::Sell ).unwrap();
  assert_eq!( restored.order.id, OrderId( 1 ), "order 1 is back, unchanged" );
  assert_eq!( restored.order.price, Price::parse( "2.50" ).unwrap() );
  assert_eq!( restored.remaining, qty( 4 ) );
}

/// Same rollback path, reached through the other refusal cause: a
/// zero-remaining replacement.
#[ test ]
fn rest_replace_refused_by_zero_quantity_restores_the_original()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Sell, "2.50", 4, 10 ) ) );

  let zero = Resting { remaining : Quantity::ZERO, ..resting( 1, Side::Sell, "2.60", 6, 20 ) };
  let error = rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), zero ).unwrap_err();

  assert_eq!( error, RestReplaceError::Refused );
  let restored = book.best( InstrumentId( 1 ), Side::Sell ).unwrap();
  assert_eq!( restored.order.price, Price::parse( "2.50" ).unwrap(), "rolled back to the pre-replace order" );
  assert_eq!( restored.remaining, qty( 4 ) );
}

// --- rest_replace: InstrumentMismatch ------------------------------------

// BUG-001 substrate/task/exchange_rest/bug/completed/001_rest_replace_trusts_instrument.md — bug_reproducer: `new_resting`'s
// own `order.instrument` disagreeing with the `instrument` parameter used to go unnoticed —
// the old order was cancelled on one instrument while the new one was inserted onto a
// different one entirely, with `rest_replace` returning `Ok`.
///
/// # Root Cause
///
/// `rest_replace` read its `instrument` parameter only for the `book.cancel` lookup;
/// the subsequent `book.insert( new_resting )` derives its target book purely from
/// `new_resting.order.instrument`, with no comparison between the two anywhere in the
/// function (`module/exchange_rest/src/lib.rs:133-151`, pre-fix).
///
/// # Why Not Caught
///
/// Every pre-existing test's `order()`/`resting()` helper hardcoded `InstrumentId( 1 )`
/// for both the explicit `instrument` argument and the constructed order, so the two
/// values were structurally identical in every call this crate's own suite made —
/// no test ever supplied two disagreeing values.
///
/// # Fix Applied
///
/// `rest_replace` now checks `new_resting.order.instrument == instrument` first, before
/// calling `book.cancel`, returning `RestReplaceError::InstrumentMismatch` on disagreement
/// so nothing is touched (`module/exchange_rest/src/lib.rs:133-145`).
///
/// # Prevention
///
/// Any function taking both an explicit selector and a payload carrying its own copy of
/// the same identity must assert the two agree before either drives an effectful
/// operation — see `substrate/task/exchange_rest/bug/completed/001_rest_replace_trusts_instrument.md § Prevention`.
///
/// # Pitfall
///
/// A lookup keyed on one value and a write keyed on a different, untrusted value that is
/// assumed to agree with it is a silent cross-target hazard — assert agreement first.
#[ test ]
fn rest_replace_refuses_a_new_resting_for_a_different_instrument()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Sell, "2.50", 4, 10 ) ) );

  let mismatched = Resting { order : Order { instrument : InstrumentId( 2 ), ..resting( 1, Side::Sell, "2.60", 6, 20 ).order }, ..resting( 1, Side::Sell, "2.60", 6, 20 ) };
  let error = rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), mismatched ).unwrap_err();

  assert_eq!( error, RestReplaceError::InstrumentMismatch );
  assert_eq!( book.best( InstrumentId( 2 ), Side::Sell ), None, "the mismatched replacement must not land on instrument 2" );
  let restored = book.best( InstrumentId( 1 ), Side::Sell ).unwrap();
  assert_eq!( restored.order.id, OrderId( 1 ), "order 1 is back on its own instrument, unchanged" );
  assert_eq!( restored.order.price, Price::parse( "2.50" ).unwrap() );
}
