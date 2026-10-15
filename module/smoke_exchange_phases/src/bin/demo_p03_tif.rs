//! Phase P03 — GTC rests, IOC does not, FOK requires full. Golden: `gtc=1 ioc=0 fok=1` then `ok`.

use exchange_tif::{ Tif, tif_requires_full, tif_rests };

fn main()
{
  let gtc = tif_rests( Tif::Gtc );
  let ioc = tif_rests( Tif::Ioc );
  let fok = tif_requires_full( Tif::Fok );

  println!( "gtc={} ioc={} fok={}", i32::from( gtc ), i32::from( ioc ), i32::from( fok ) );
  assert!( gtc && !ioc && fok );
  println!( "ok" );
}
