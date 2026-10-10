//! Halt and resume one instrument: while halted, `exchange_core` refuses
//! new orders on it; resting orders and cancels are untouched.
//!
//! The flag is `InstrumentSpec::halted`; this crate guards its transitions.
//! `Exchange::halt_set`/`halt_clear`/`halt_is` call straight through, and
//! `step_place` refuses an order on a halted instrument with
//! `RejectReason::Halted`.
//!
//! Depends on `exchange_spec` only, not the source design's `exchange_book` —
//! see `docs/decisions/001_no_exchange_book_dependency.md`.

use exchange_spec::{ InstrumentSpec, spec_halted_is };

/// Why a halt/resume request was refused.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum HaltError
{
  /// The instrument was already in the requested state — halting an
  /// already-halted instrument, or resuming one that was never halted.
  Already,
}

impl core::fmt::Display for HaltError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::Already => write!( f, "the instrument is already in the requested state" ),
    }
  }
}

impl core::error::Error for HaltError {}

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

/// Whether `spec`'s instrument is currently halted —
/// [`exchange_spec::spec_halted_is`] under the source design's name.
#[ must_use ]
pub fn halt_is( spec : &InstrumentSpec ) -> bool
{
  spec_halted_is( spec )
}
