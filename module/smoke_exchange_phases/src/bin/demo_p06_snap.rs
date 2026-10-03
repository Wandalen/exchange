//! Phase P06 — 1.26 snapped to a 0.05 tick grid lands on 1.25. Golden: `p=1.25` then `ok`.

use exact_arith::{ Price, Quantity, price_fmt };
use exchange_id::InstrumentId;
use exchange_spec::{ AssetId, price_snap, spec_new };

fn main()
{
  let spec = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();
  let snapped = price_snap( &spec, Price::parse( "1.26" ).unwrap() ).unwrap();

  println!( "p={}", price_fmt( snapped ) );
  assert_eq!( snapped, Price::parse( "1.25" ).unwrap() );
  println!( "ok" );
}
