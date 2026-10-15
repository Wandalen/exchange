//! Test Matrix T01 — the three dispositions, and Phase P03's own smoke assertion.

use exchange_tif::{ Tif, tif_requires_full, tif_rests };

/// T01 — only GTC rests a remainder.
#[ test ]
fn only_gtc_rests()
{
  assert!( tif_rests( Tif::Gtc ) );
  assert!( !tif_rests( Tif::Ioc ) );
  assert!( !tif_rests( Tif::Fok ) );
}

/// T01 — only FOK demands a complete fill.
#[ test ]
fn only_fok_requires_full()
{
  assert!( !tif_requires_full( Tif::Gtc ) );
  assert!( !tif_requires_full( Tif::Ioc ) );
  assert!( tif_requires_full( Tif::Fok ) );
}

/// Phase P03 — GTC rests, IOC does not, FOK requires full, in one pass.
///
/// Golden print: `gtc=1 ioc=0 fok=1` then `ok` (`docs/golden_output/003_p03_golden.md`).
#[ test ]
fn p03_tif_disposition()
{
  let gtc = tif_rests( Tif::Gtc );
  let ioc = tif_rests( Tif::Ioc );
  let fok = tif_requires_full( Tif::Fok );

  assert!( gtc && !ioc && fok );
}
