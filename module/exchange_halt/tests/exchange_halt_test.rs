//! Test Matrix T01 — halt/resume toggling and the `Already` no-op guard.

use exact_arith::{ Price, Quantity };
use exchange_halt::{ HaltError, halt_clear, halt_is, halt_set };
use exchange_id::InstrumentId;
use exchange_spec::{ AssetId, InstrumentSpec, spec_new };

fn spec() -> InstrumentSpec
{
  spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap()
}

/// T01 — a freshly-built spec starts unhalted.
#[ test ]
fn fresh_spec_starts_unhalted()
{
  assert!( !halt_is( &spec() ) );
}

/// T01 — halting an unhalted instrument succeeds and is observable.
#[ test ]
fn halt_set_halts_an_unhalted_spec()
{
  let mut spec = spec();
  assert_eq!( halt_set( &mut spec ), Ok( () ) );
  assert!( halt_is( &spec ) );
}

/// T01 — halting an already-halted instrument is refused, not silently accepted.
#[ test ]
fn halt_set_refuses_an_already_halted_spec()
{
  let mut spec = spec();
  assert_eq!( halt_set( &mut spec ), Ok( () ) );
  assert_eq!( halt_set( &mut spec ), Err( HaltError::Already ) );
  assert!( halt_is( &spec ), "the refused second halt must leave the instrument halted, not toggle it off" );
}

/// T01 — resuming a halted instrument succeeds and is observable.
#[ test ]
fn halt_clear_resumes_a_halted_spec()
{
  let mut spec = spec();
  assert_eq!( halt_set( &mut spec ), Ok( () ) );
  assert_eq!( halt_clear( &mut spec ), Ok( () ) );
  assert!( !halt_is( &spec ) );
}

/// T01 — resuming an instrument that was never halted is refused.
#[ test ]
fn halt_clear_refuses_an_unhalted_spec()
{
  let mut spec = spec();
  assert_eq!( halt_clear( &mut spec ), Err( HaltError::Already ) );
  assert!( !halt_is( &spec ), "the refused resume must leave the instrument unhalted, not toggle it on" );
}

/// `halt_is` agrees with `exchange_spec::spec_halted_is` on the same spec —
/// the two must never diverge, since `halt_is` is a wrapper over it.
#[ test ]
fn halt_is_agrees_with_spec_halted_is()
{
  let mut spec = spec();
  assert_eq!( halt_is( &spec ), exchange_spec::spec_halted_is( &spec ) );

  halt_set( &mut spec ).unwrap();
  assert_eq!( halt_is( &spec ), exchange_spec::spec_halted_is( &spec ) );
}

/// `HaltError` reads as a sentence and works as an error source, like every
/// other error type in the family.
#[ test ]
fn halt_error_displays_and_is_an_error()
{
  assert_eq!( HaltError::Already.to_string(), "the instrument is already in the requested state" );
  let _ : &dyn core::error::Error = &HaltError::Already;
}
