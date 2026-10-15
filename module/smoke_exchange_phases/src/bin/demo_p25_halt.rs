//! Phase P25 — halt then resume round-trips correctly. Golden: `halt=1 resume=1` then `ok`.
//!
//! Adapted from the design transcript's own framing ("placement was blocked
//! while halted and worked again after resume"): `exchange_halt` isn't wired
//! into any placement path yet — that belongs to whichever stage reworks
//! `exchange_match`/`exchange_rest` (see `exchange_halt`'s own
//! `docs/decisions/001_no_exchange_book_dependency.md`). This phase instead
//! confirms the contract `exchange_halt` actually owns today: `halt_set`
//! takes effect (`halt=1`) and `halt_clear` takes it back (`resume=1`). The
//! golden line's literal text is unchanged from the transcript; only what it
//! measures is.

use exact_arith::{ Price, Quantity };
use exchange_halt::{ halt_clear, halt_is, halt_set };
use exchange_id::InstrumentId;
use exchange_spec::{ AssetId, spec_new };

fn main()
{
  let mut spec = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();

  halt_set( &mut spec ).unwrap();
  let halt = i32::from( halt_is( &spec ) );

  halt_clear( &mut spec ).unwrap();
  let resume = i32::from( !halt_is( &spec ) );

  println!( "halt={halt} resume={resume}" );
  assert_eq!( ( halt, resume ), ( 1, 1 ) );
  println!( "ok" );
}
