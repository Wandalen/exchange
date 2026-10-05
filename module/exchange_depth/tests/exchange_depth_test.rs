//! Test Matrix T01 — depth aggregation: per-level summing, top-N truncation,
//! the `BadN` guard, and independence from book mutation after the read.

use exact_arith::{ Money, Quantity };
use exchange_book::{ Book, Resting };
use exchange_depth::{ Depth, DepthError, LevelView, depth_top };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

const INSTRUMENT : InstrumentId = InstrumentId( 1 );

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

fn level( price : &str, qty : i64 ) -> LevelView
{
  LevelView { price : Money::parse( price ).unwrap(), qty : Quantity::from_int( qty ).unwrap() }
}

/// T01 — an empty book has no depth on either side, whatever `n` is asked for.
#[ test ]
fn empty_book_has_no_depth()
{
  let book = Book::new();
  assert_eq!( depth_top( &book, INSTRUMENT,5 ).unwrap(), Depth::default() );
}

/// T01 — `n == 0` is refused; nothing else is.
#[ test ]
fn zero_n_is_refused()
{
  let book = Book::new();
  assert_eq!( depth_top( &book, INSTRUMENT,0 ), Err( DepthError::BadN ) );
}

/// T01 — orders resting at the same price aggregate into one level.
#[ test ]
fn same_price_orders_aggregate_into_one_level()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "1.00", 3, 10 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 2, Side::Buy, "1.00", 4, 20 ) ), "fresh id, must succeed" );

  let depth = depth_top( &book, INSTRUMENT,5 ).unwrap();
  assert_eq!( depth.bids, vec![ level( "1.00", 7 ) ], "two orders at one price, one summed level" );
}

/// T01 — distinct prices stay distinct levels, best first.
#[ test ]
fn distinct_prices_stay_distinct_levels_best_first()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "1.00", 3, 10 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 2, Side::Buy, "2.00", 4, 20 ) ), "fresh id, must succeed" );

  let depth = depth_top( &book, INSTRUMENT,5 ).unwrap();
  assert_eq!( depth.bids, vec![ level( "2.00", 4 ), level( "1.00", 3 ) ], "higher bid first" );
}

/// T01 — a request for more levels than exist returns fewer, not an error.
#[ test ]
fn requesting_more_than_exists_returns_fewer_not_an_error()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "1.00", 3, 10 ) ), "fresh id, must succeed" );

  let depth = depth_top( &book, INSTRUMENT,5 ).unwrap();
  assert_eq!( depth.bids.len(), 1, "only n == 0 is an error, not n too large" );
}

/// T01 — a third distinct price is truncated once `n` levels are already full.
#[ test ]
fn truncates_to_n_distinct_levels()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "3.00", 1, 10 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 2, Side::Buy, "2.00", 1, 20 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 3, Side::Buy, "1.00", 1, 30 ) ), "fresh id, must succeed" );

  let depth = depth_top( &book, INSTRUMENT,2 ).unwrap();
  assert_eq!( depth.bids, vec![ level( "3.00", 1 ), level( "2.00", 1 ) ], "the third, worse level is dropped" );
}

/// T01 — a fourth order at an already-truncated level is still dropped with
/// it — truncation is by distinct price, not by order count.
#[ test ]
fn truncation_drops_every_order_behind_the_cut_price()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "3.00", 1, 10 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 2, Side::Buy, "2.00", 1, 20 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 3, Side::Buy, "1.00", 1, 30 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 4, Side::Buy, "1.00", 9, 40 ) ), "fresh id, must succeed" );

  let depth = depth_top( &book, INSTRUMENT,2 ).unwrap();
  assert_eq!( depth.bids, vec![ level( "3.00", 1 ), level( "2.00", 1 ) ], "both orders at 1.00 are past the cut" );
}

/// T01 — the two sides are reported independently; asks are lowest-first.
#[ test ]
fn both_sides_reported_independently()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "1.00", 5, 10 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 2, Side::Sell, "2.00", 5, 20 ) ), "fresh id, must succeed" );
  assert!( book.insert( rest( 3, Side::Sell, "1.50", 5, 30 ) ), "fresh id, must succeed" );

  let depth = depth_top( &book, INSTRUMENT,5 ).unwrap();
  assert_eq!( depth.bids, vec![ level( "1.00", 5 ) ] );
  assert_eq!( depth.asks, vec![ level( "1.50", 5 ), level( "2.00", 5 ) ], "lowest ask first" );
}

/// A `Depth` is a snapshot, not a view: cancelling afterward never reaches
/// back into an already-returned value.
#[ test ]
fn depth_is_a_snapshot_not_a_view()
{
  let mut book = Book::new();
  assert!( book.insert( rest( 1, Side::Buy, "1.00", 5, 10 ) ), "fresh id, must succeed" );

  let depth = depth_top( &book, INSTRUMENT,5 ).unwrap();
  assert!( book.cancel( INSTRUMENT, OrderId( 1 ) ).is_some() );

  assert_eq!( depth.bids, vec![ level( "1.00", 5 ) ], "the live cancel must not reach the already-taken depth" );
}
