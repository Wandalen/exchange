//! An on/off switch for matching alone — resting orders are never touched.
//!
//! Without this crate there is no way to stop a market: no circuit breaker,
//! no station lockdown. Closes hard problem 18 (halt) and feature 18
//! (`book_halt`/`book_resume`).
//!
//! # Only `exchange_spec`, not `exchange_book`
//!
//! The source design names `exchange_book` as a second dependency. Nothing
//! in the real match loop checks `InstrumentSpec::halted` yet — that wiring
//! belongs to whichever stage reworks `exchange_match`/`exchange_rest`, the
//! same place `exchange_cap`'s own `BookCaps` is still waiting to be
//! consulted. Taking the dependency now would buy nothing: there is no real
//! behavior today it would let this crate add. See
//! [`docs/decisions`](docs/decisions) for the full reasoning.

use exchange_spec::{ InstrumentSpec, spec_halted_is };

/// Why a halt/resume request was refused.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum HaltError
{
  /// The instrument was already in the requested state — halting an
  /// already-halted instrument, or resuming one that was never halted.
  Already,
}

/// Halt `spec`'s instrument, refusing a no-op halt.
///
/// # Errors
///
/// [`HaltError::Already`] if `spec` is already halted.
pub fn halt_set( spec : &mut InstrumentSpec ) -> Result< (), HaltError >
{
  if spec.halted
  {
    return Err( HaltError::Already );
  }
  spec.halted = true;
  Ok( () )
}

/// Resume `spec`'s instrument, refusing a no-op resume.
///
/// # Errors
///
/// [`HaltError::Already`] if `spec` is not currently halted.
pub fn halt_clear( spec : &mut InstrumentSpec ) -> Result< (), HaltError >
{
  if !spec.halted
  {
    return Err( HaltError::Already );
  }
  spec.halted = false;
  Ok( () )
}

/// Whether `spec`'s instrument is currently halted.
///
/// A thin, `exchange_halt`-named wrapper over
/// [`exchange_spec::spec_halted_is`] — giving this crate the complete
/// three-function surface the proposal names, without a second
/// implementation of the one-line read it already is.
#[ must_use ]
pub fn halt_is( spec : &InstrumentSpec ) -> bool
{
  spec_halted_is( spec )
}
