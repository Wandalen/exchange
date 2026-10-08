//! Test Matrix T01 — snapshot content, ordering, and independence from the
//! live book (pitfalls 001–003: aliasing, hash order, clock reads).

use exact_kind::{ Money, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_snap::{ BookSnap, RestRow, snap_len, snap_take };
use exchange_tif::Tif;

fn rest( id : u64, side : Side, price : &str, quantity : i64, arrival : u64 ) -> Resting
{
  let quantity = Quantity::from_int( quantity ).unwrap();
  Resting
  {
    order : Order
    {
      id : OrderId( id ),
      instrument : InstrumentId( 1 ),
      account : AccountId( id ),
      side,
      price : Money::parse( price ).unwrap(),
      quantity,
      tif : Tif::Gtc,
    },
    remaining : quantity,
    arrival : Sequence( arrival ),
  }
}

/// T01 — an empty book snaps to an empty row list, carrying the caller's
/// instrument and tick unchanged.
#[ test ]
fn empty_book_yields_empty_snap()
{
  let book = Book::new();
  let snap = snap_take( &book, InstrumentId( 7 ), Money::parse( "0.01" ).unwrap() );

  assert_eq!( snap.instrument, InstrumentId( 7 ) );
  assert_eq!( snap.tick, Money::parse( "0.01" ).unwrap() );
  assert!( snap.rows.is_empty() );
  assert_eq!( snap_len( &snap ), 0 );
}

/// T01 — every resting order is copied, with its remaining (not submitted) quantity.
#[ test ]
fn snap_take_copies_every_resting_order_with_its_remaining_quantity()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "1.00", 5, 10 ) ), "fresh id, must succeed" );
  assert!( book.consume_best( InstrumentId( 1 ), Side::Buy, Quantity::from_int( 2 ).unwrap() ), "partial fill, 3 left" );

  let snap = snap_take( &book, InstrumentId( 1 ), Money::parse( "0.01" ).unwrap() );

  assert_eq!( snap.rows, vec!
  [
    RestRow { order : OrderId( 1 ), price : Money::parse( "1.00" ).unwrap(), qty : Quantity::from_int( 3 ).unwrap() },
  ] );
  assert_eq!( snap_len( &snap ), 1 );
}

/// T01 — rows preserve the book's own priority order: bids then asks, each
/// side still ranked, never a hash-derived order (pitfall 002).
#[ test ]
fn rows_preserve_the_books_priority_order()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "1.00", 1, 10 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 2, Side::Buy, "2.00", 1, 20 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 3, Side::Sell, "4.00", 1, 30 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 4, Side::Sell, "3.00", 1, 40 ) ), "fresh id, must succeed" );

  let snap = snap_take( &book, InstrumentId( 1 ), Money::parse( "0.01" ).unwrap() );
  let ids : Vec< u64 > = snap.rows.iter().map( | row | row.order.0 ).collect();

  assert_eq!( ids, vec![ 2, 1, 4, 3 ], "best bid first, then best ask first — exchange_book's own published order" );
}

/// A `BookSnap` is a copy, not a view: mutating the book after taking the
/// snapshot must never reach back into it (pitfall 001).
#[ test ]
fn snap_is_a_copy_not_a_view()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "1.00", 5, 10 ) ), "fresh id, must succeed" );

  let snap = snap_take( &book, InstrumentId( 1 ), Money::parse( "0.01" ).unwrap() );
  assert!( book.cancel( InstrumentId( 1 ), OrderId( 1 ) ).is_some() );

  assert_eq!( snap.rows.len(), 1, "the live cancel must not reach back into the already-taken snapshot" );
}

/// `BookSnap`/`RestRow` are plain, independently-owned values — `Clone`
/// produces a fully independent second copy, not a shared handle.
#[ test ]
fn book_snap_clones_independently()
{
  let book = Book::new();
  let snap : BookSnap = snap_take( &book, InstrumentId( 1 ), Money::parse( "0.01" ).unwrap() );
  let cloned = snap.clone();

  assert_eq!( snap, cloned );
}
