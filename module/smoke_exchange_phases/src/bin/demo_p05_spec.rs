//! Phase P05 — a valid spec builds; a zero tick is refused. Golden: `ok=1 zt=1` then `ok`.

use exact_arith::{ Price, Quantity };
use exchange_id::InstrumentId;
use exchange_spec::{ AssetId, spec_new };

fn main()
{
  let ok = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).is_ok();
  let zt = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.00" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).is_err();

  println!( "ok={} zt={}", u8::from( ok ), u8::from( zt ) );
  assert!( ok && zt );
  println!( "ok" );
}
