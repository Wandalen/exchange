//! A limit on how large a book may grow, with a named refusal past it.
//!
//! A root of the dependency tree: no dependency on any other `exchange_*`
//! crate, and this crate does not itself match or walk the book — it is the
//! one check `exchange_rest` consults before accepting a new resting order
//! or opening a new price level. Without a cap, a book either grows without
//! bound or silently drops an order nothing else reports as missing.
//!
//! Genuinely new: no capacity limit exists anywhere in the real crates today.
//! This closes hard problem 17 (capacity) and feature 20 (`BookCaps`, `Full`).

/// The two limits a book may be configured with.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct BookCaps
{
  /// The most resting orders one price level may hold.
  pub max_rests : usize,
  /// The most distinct price levels one side of the book may hold.
  pub max_levels : usize,
}

/// Why a cap check refused.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum CapError
{
  /// The level already holds `caps.max_rests` resting orders.
  RestsFull,
  /// The side already holds `caps.max_levels` distinct price levels.
  LevelsFull,
}

impl core::fmt::Display for CapError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::RestsFull => write!( f, "the level is already at its configured rest capacity" ),
      Self::LevelsFull => write!( f, "the side is already at its configured level capacity" ),
    }
  }
}

impl core::error::Error for CapError {}

/// Whether one more resting order may join a level already holding
/// `current_rests`, under `caps`.
///
/// # Errors
///
/// [`CapError::RestsFull`] if `current_rests` has already reached
/// `caps.max_rests`.
pub const fn cap_check_rest( caps : BookCaps, current_rests : usize ) -> Result< (), CapError >
{
  if current_rests >= caps.max_rests
  {
    return Err( CapError::RestsFull );
  }

  Ok( () )
}

/// Whether one more distinct price level may open on a side already holding
/// `current_levels`, under `caps`.
///
/// # Errors
///
/// [`CapError::LevelsFull`] if `current_levels` has already reached
/// `caps.max_levels`.
pub const fn cap_check_level( caps : BookCaps, current_levels : usize ) -> Result< (), CapError >
{
  if current_levels >= caps.max_levels
  {
    return Err( CapError::LevelsFull );
  }

  Ok( () )
}
