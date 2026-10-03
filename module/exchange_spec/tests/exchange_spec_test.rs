//! Test Matrix T01 — spec construction and the snap grid, plus Phases P05/P06.

use exact_arith::{ Price, Quantity };
use exchange_id::InstrumentId;
use exchange_spec::{ AssetId, SpecError, price_snap, qty_snap, spec_halted_is, spec_new };

fn spec() -> exchange_spec::InstrumentSpec
{
  spec_new
  (
    InstrumentId( 1 ),
    AssetId( 1 ),
    AssetId( 2 ),
    Price::parse( "0.05" ).unwrap(),
    Quantity::from_int( 1 ).unwrap(),
  )
  .unwrap()
}

/// T01 — a non-degenerate tick and lot construct successfully, un-halted.
#[ test ]
fn a_valid_spec_constructs_un_halted()
{
  assert!( !spec_halted_is( &spec() ) );
}

/// T01 — a zero tick is refused, not silently accepted.
#[ test ]
fn a_zero_tick_is_refused()
{
  let err = spec_new
  (
    InstrumentId( 1 ),
    AssetId( 1 ),
    AssetId( 2 ),
    Price::parse( "0.00" ).unwrap(),
    Quantity::from_int( 1 ).unwrap(),
  );

  assert_eq!( err, Err( SpecError::ZeroTick ) );
}

/// T01 — a zero lot is refused, not silently accepted.
#[ test ]
fn a_zero_lot_is_refused()
{
  let err = spec_new
  (
    InstrumentId( 1 ),
    AssetId( 1 ),
    AssetId( 2 ),
    Price::parse( "0.05" ).unwrap(),
    Quantity::from_int( 0 ).unwrap(),
  );

  assert_eq!( err, Err( SpecError::ZeroLot ) );
}

/// T01 — `base`/`quote` land in the field matching their own parameter name,
/// not swapped — the two parameters share a type, so a swap compiles clean.
#[ test ]
fn base_and_quote_are_not_swapped()
{
  let s = spec_new( InstrumentId( 1 ), AssetId( 11 ), AssetId( 22 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();
  assert_eq!( s.base, AssetId( 11 ) );
  assert_eq!( s.quote, AssetId( 22 ) );
}

/// T01 — two different instruments' ids and assets are not conflated.
#[ test ]
fn distinct_instruments_keep_distinct_ids_and_assets()
{
  let a = spec();
  let b = spec_new
  (
    InstrumentId( 2 ),
    AssetId( 3 ),
    AssetId( 4 ),
    Price::parse( "0.01" ).unwrap(),
    Quantity::from_int( 1 ).unwrap(),
  )
  .unwrap();

  assert_ne!( a.id, b.id );
  assert_ne!( a.base, b.base );
}

/// Phase P05 — tick 0.05/lot 1 construct; a zero tick is an error, not a
/// silently-accepted default.
///
/// Golden print: `ok` (`docs/golden_output`'s P05 entry).
#[ test ]
fn p05_spec_validity()
{
  assert!( spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).is_ok() );
  assert!( spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.00" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).is_err() );
}

/// Phase P06 — 1.26 snapped to a 0.05 tick lands on 1.25.
///
/// Golden print: `p=1.25` then `ok` (`docs/golden_output/006_p06_golden.md`).
#[ test ]
fn p06_price_snap()
{
  let snapped = price_snap( &spec(), Price::parse( "1.26" ).unwrap() ).unwrap();
  assert_eq!( snapped, Price::parse( "1.25" ).unwrap() );
}

/// `qty_snap` is the same wrapper shape as `price_snap`, over the lot grid.
#[ test ]
fn qty_snap_rounds_to_the_lot_grid()
{
  let s = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 5 ).unwrap() ).unwrap();
  let snapped = qty_snap( &s, Quantity::from_int( 12 ).unwrap() ).unwrap();
  assert_eq!( snapped, Quantity::from_int( 10 ).unwrap() );
}
