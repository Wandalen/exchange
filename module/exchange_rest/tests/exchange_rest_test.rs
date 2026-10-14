//! Test Matrix — the three non-matching ways an order moves on the book:
//! `rest_place`, `rest_cancel`, `rest_replace`.
//!
//! `rest_place`/`rest_cancel` are direct call-throughs to `Book::insert`/
//! `Book::cancel`, already exhaustively covered by `exchange_book`'s own
//! test suite — these just confirm the wrapper forwards faithfully, not a
//! second copy of `Book`'s own matrix. `rest_replace` is this crate's one
//! genuinely new operation and gets the deeper coverage, including the
//! refusal a plain cancel-then-insert can't make atomic.

use exact_arith::{ Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, ClientOrderId, InstrumentId, OrderId };
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
/// to whatever already rested at the better price.
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

// --- rest_replace: Refused ----------------------------------------------

/// What this crate exists to provide: a replacement that collides with a
/// third order is refused, and the original stays exactly as it was rather
/// than staying cancelled.
#[ test ]
fn rest_replace_refused_by_a_colliding_id_leaves_the_original()
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
  assert_eq!( restored.order.id, OrderId( 1 ), "order 1 still rests, unchanged" );
  assert_eq!( restored.order.price, Price::parse( "2.50" ).unwrap() );
  assert_eq!( restored.remaining, qty( 4 ) );
}

/// Same refusal, reached through the other cause: a zero-remaining
/// replacement.
#[ test ]
fn rest_replace_refused_by_zero_quantity_leaves_the_original()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Sell, "2.50", 4, 10 ) ) );

  let zero = Resting { remaining : Quantity::ZERO, ..resting( 1, Side::Sell, "2.60", 6, 20 ) };
  let error = rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), zero ).unwrap_err();

  assert_eq!( error, RestReplaceError::Refused );
  let restored = book.best( InstrumentId( 1 ), Side::Sell ).unwrap();
  assert_eq!( restored.order.price, Price::parse( "2.50" ).unwrap(), "still the pre-replace order" );
  assert_eq!( restored.remaining, qty( 4 ) );
}

/// A refused replace keeps the original's place among orders sharing its
/// arrival.
///
/// # Root Cause
///
/// `rest_replace` cancelled the original, then reinserted it when the
/// replacement was refused. `Book::insert` seats an order behind every peer
/// sharing its arrival, so it came back behind those it had been ahead of.
///
/// # Why Not Caught
///
/// Every refusal test rested one order per arrival.
///
/// # Fix Applied
///
/// `rest_replace` asks `Book::insertable` first and touches nothing on a
/// refusal.
///
/// # Prevention
///
/// "Put back exactly as it was" is checked against the queue, not the order.
///
/// # Pitfall
///
/// Only ties expose it, and only `exchange_inbound::inbound_apply` rests
/// arrivals it did not claim itself.
#[ test ]
fn rest_replace_refused_keeps_the_originals_place_among_shared_arrivals()
{
  let mut book = Book::new();
  for id in 1..=3
  {
    assert!( rest_place( &mut book, resting( id, Side::Sell, "2.50", 4, 10 ) ) );
  }
  let queue = | book : &Book | book.side( InstrumentId( 1 ), Side::Sell ).map( | resting | resting.order.id.0 ).collect::< Vec< _ > >();

  for id in [ 1, 2 ]
  {
    let zero = Resting { remaining : Quantity::ZERO, ..resting( id, Side::Sell, "2.60", 4, 20 ) };
    assert_eq!( rest_replace( &mut book, InstrumentId( 1 ), OrderId( id ), zero ), Err( RestReplaceError::Refused ) );
    assert_eq!( queue( &book ), vec![ 1, 2, 3 ], "order {id} keeps its place" );
  }
}

// --- rest_replace: InstrumentMismatch ------------------------------------

/// A replacement naming another instrument is refused, and the original
/// stays on its own.
///
/// # Root Cause
///
/// `rest_replace` read `instrument` only to find the original; `Book::insert`
/// seats the replacement on its own order's instrument. The two were never
/// compared, so the original was cancelled on one book and the replacement
/// rested on another, with `Ok`.
///
/// # Why Not Caught
///
/// Every test built both values from one hardcoded `InstrumentId( 1 )`.
///
/// # Fix Applied
///
/// `rest_replace` compares them first and returns
/// `RestReplaceError::InstrumentMismatch`.
///
/// # Prevention
///
/// When a function takes one identity twice, test it with the two apart.
///
/// # Pitfall
///
/// A lookup keyed on one value and a write keyed on another, assumed equal,
/// acts on a target nobody checked.
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

// --- rest_replace: WouldTake ---------------------------------------------

/// A replacement priced into the opposite side is refused, and the original
/// stays.
///
/// # Root Cause
///
/// `rest_replace` checked only what `Book::insert` checks. A bid moved above
/// the best ask rested there untraded, leaving the book crossed: the next
/// seller traded against that bid at its price while the cheaper ask sat on.
///
/// # Why Not Caught
///
/// Every replacement in the suite moved an order away from the other side.
///
/// # Fix Applied
///
/// `rest_replace` refuses a replacement whose price reaches the best opposite
/// order with `RestReplaceError::WouldTake`.
///
/// # Prevention
///
/// A move onto the book that does not match must be one that could not have.
///
/// # Pitfall
///
/// The opposite side counts every account: a crossing against one's own
/// order is still a crossed book.
#[ test ]
fn rest_replace_refuses_a_replacement_that_would_take()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Buy, "2.40", 4, 10 ) ) );
  assert!( rest_place( &mut book, resting( 2, Side::Sell, "2.50", 4, 20 ) ) );

  for price in [ "2.50", "2.60" ]
  {
    let error = rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), resting( 1, Side::Buy, price, 4, 30 ) ).unwrap_err();
    assert_eq!( error, RestReplaceError::WouldTake, "a bid at {price} reaches the 2.50 ask" );
  }

  let bid = book.best( InstrumentId( 1 ), Side::Buy ).unwrap();
  assert_eq!( bid.order.price, Price::parse( "2.40" ).unwrap(), "the original bid still rests" );
  assert_eq!( bid.arrival, Sequence( 10 ) );
}

/// A replacement one tick short of the opposite side is not refused.
#[ test ]
fn rest_replace_up_to_the_opposite_side_is_accepted()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Buy, "2.40", 4, 10 ) ) );
  assert!( rest_place( &mut book, resting( 2, Side::Sell, "2.50", 4, 20 ) ) );

  rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), resting( 1, Side::Buy, "2.49", 4, 30 ) ).unwrap();

  assert_eq!( book.best( InstrumentId( 1 ), Side::Buy ).unwrap().order.price, Price::parse( "2.49" ).unwrap() );
}

// --- rest_replace: NotAnAmend --------------------------------------------

/// A replacement with another side, account or time in force is refused,
/// and the original stays.
///
/// # Root Cause
///
/// Nothing compared the replacement with the order it replaces. A replace
/// could turn a bid into an ask, hand the order and its place in the queue to
/// another account, or make it an IOC left resting on the book.
///
/// # Why Not Caught
///
/// Every replacement in the suite came from the same helper as its original,
/// differing only in price and quantity.
///
/// # Fix Applied
///
/// `rest_replace` looks the original up first and refuses any of the three
/// changes with `RestReplaceError::NotAnAmend`.
///
/// # Prevention
///
/// `exchange_core/docs/algorithm/003_cancel_and_amend_semantics.md` lists
/// what an amend may change; test each thing it may not.
///
/// # Pitfall
///
/// The id may change in a replace, so it cannot be what says "the same order".
#[ test ]
fn rest_replace_refuses_a_replacement_that_is_another_order()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Sell, "2.50", 4, 10 ) ) );
  let replacement = resting( 1, Side::Sell, "2.60", 4, 20 );

  let other_side = Resting { order : Order { side : Side::Buy, price : Price::parse( "2.40" ).unwrap(), ..replacement.order }, ..replacement };
  let other_account = Resting { order : Order { account : AccountId( 9 ), ..replacement.order }, ..replacement };
  let ioc = Resting { order : Order { tif : Tif::Ioc, ..replacement.order }, ..replacement };

  for changed in [ other_side, other_account, ioc ]
  {
    assert_eq!( rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), changed ), Err( RestReplaceError::NotAnAmend ) );
    let original = book.best( InstrumentId( 1 ), Side::Sell ).unwrap();
    assert_eq!( ( original.order.price, original.order.tif ), ( Price::parse( "2.50" ).unwrap(), Tif::Gtc ) );
    assert_eq!( book.len(), 1 );
  }
}

/// A replacement under another client id is refused, and the original
/// stays.
///
/// # Root Cause
///
/// The client id was left out of the comparison with the original. A caller
/// that claims client ids per order, as `exchange_inbound::inbound_apply`
/// does, never claimed the new one, so a later order could reuse it while the
/// replacement rested under it.
///
/// # Why Not Caught
///
/// Every order in the suite has no client id.
///
/// # Fix Applied
///
/// `rest_replace` compares the client id along with side, account and time in
/// force, and refuses a change with `RestReplaceError::NotAnAmend`.
///
/// # Prevention
///
/// Test each field that names an order to its owner, not only the book's.
///
/// # Pitfall
///
/// Dropping the client id is a change too.
#[ test ]
fn rest_replace_refuses_a_replacement_under_another_client_id()
{
  let mut book = Book::new();
  let with_client = | resting : Resting, client | Resting { order : Order { client, ..resting.order }, ..resting };
  let original = with_client( resting( 1, Side::Sell, "2.50", 4, 10 ), Some( ClientOrderId( 5 ) ) );
  assert!( rest_place( &mut book, original ) );

  for client in [ Some( ClientOrderId( 6 ) ), None ]
  {
    let changed = with_client( resting( 1, Side::Sell, "2.60", 4, 20 ), client );
    assert_eq!( rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), changed ), Err( RestReplaceError::NotAnAmend ) );
    assert_eq!( book.best( InstrumentId( 1 ), Side::Sell ).unwrap().order, original.order );
  }

  let kept = with_client( resting( 1, Side::Sell, "2.60", 4, 20 ), Some( ClientOrderId( 5 ) ) );
  rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), kept ).unwrap();
}

/// A replacement under a new id is still the same order.
#[ test ]
fn rest_replace_may_rename_the_order()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Sell, "2.50", 4, 10 ) ) );
  let renamed = Resting { order : Order { id : OrderId( 7 ), ..resting( 1, Side::Sell, "2.60", 4, 20 ).order }, ..resting( 1, Side::Sell, "2.60", 4, 20 ) };

  rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), renamed ).unwrap();

  assert_eq!( book.best( InstrumentId( 1 ), Side::Sell ).unwrap().order.id, OrderId( 7 ) );
  assert_eq!( book.len(), 1 );
}

// --- rest_replace: JumpsQueue --------------------------------------------

/// A replacement that raises its quantity or moves its price must arrive
/// after the original, and one that keeps both no earlier than it.
///
/// # Root Cause
///
/// `rest_replace` took the replacement's arrival as given. Carrying the
/// original's, a replacement kept the original's place whatever it changed,
/// though the amend rule gives that place up for a larger quantity or
/// another price.
///
/// # Why Not Caught
///
/// Every replacement in the suite carried a later arrival.
///
/// # Fix Applied
///
/// `rest_replace` refuses an arrival the rule does not allow with
/// `RestReplaceError::JumpsQueue`.
///
/// # Prevention
///
/// One case per row of the priority table in
/// `exchange_core/docs/algorithm/003_cancel_and_amend_semantics.md`.
///
/// # Pitfall
///
/// `rest_replace` cannot tell which later arrival is right, only that an
/// amend never arrives before the order it amends.
#[ test ]
fn rest_replace_refuses_a_replacement_that_jumps_the_queue()
{
  let mut book = Book::new();
  assert!( rest_place( &mut book, resting( 1, Side::Sell, "2.50", 4, 10 ) ) );
  assert!( rest_place( &mut book, resting( 2, Side::Sell, "2.50", 4, 20 ) ) );
  assert!( rest_place( &mut book, resting( 3, Side::Sell, "2.40", 4, 15 ) ) );
  assert!( rest_place( &mut book, resting( 4, Side::Sell, "2.50", 4, 7 ) ) );
  let queue = | book : &Book | book.side( InstrumentId( 1 ), Side::Sell ).map( | resting | resting.order.id.0 ).collect::< Vec< _ > >();
  assert_eq!( queue( &book ), vec![ 3, 4, 1, 2 ] );

  // Ahead of order 2 with more to sell, ahead of order 3 at its price, ahead
  // of order 4 with less.
  for ( price, quantity, arrival ) in [ ( "2.50", 5, 10 ), ( "2.40", 4, 10 ), ( "2.50", 3, 5 ) ]
  {
    let error = rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), resting( 1, Side::Sell, price, quantity, arrival ) ).unwrap_err();
    assert_eq!( error, RestReplaceError::JumpsQueue, "{quantity} at {price}, arriving at {arrival}" );
    assert_eq!( queue( &book ), vec![ 3, 4, 1, 2 ] );
  }

  rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), resting( 1, Side::Sell, "2.50", 3, 10 ) ).unwrap();
  assert_eq!( queue( &book ), vec![ 3, 4, 1, 2 ], "a smaller quantity at one price keeps its place" );
  rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), resting( 1, Side::Sell, "2.50", 5, 30 ) ).unwrap();
  assert_eq!( queue( &book ), vec![ 3, 4, 2, 1 ], "a larger one, arriving later, goes behind order 2" );
}
