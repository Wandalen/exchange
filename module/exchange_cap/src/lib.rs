//! A limit on how large a book may grow, with a named refusal past it.
//!
//! A root of the dependency tree. This crate only checks: `exchange_core`
//! calls [`cap_check_rest`]/[`cap_check_level`]/[`cap_check_account`] before
//! an order rests and reports a refusal as
//! `RejectReason::RestsFull`/`LevelsFull`/`AccountFull`. Without a cap, a book
//! either grows without bound or silently drops an order, and one account can
//! fill it alone.

/// The three limits a book may be configured with.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct BookCaps
{
  /// The most resting orders one price level may hold.
  pub max_rests : usize,
  /// The most distinct price levels one side of the book may hold.
  pub max_levels : usize,
  /// The most resting orders one account may hold on the book, both sides
  /// together.
  pub max_account_rests : usize,
}

/// Why a cap check refused.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum CapError
{
  /// The level already holds `caps.max_rests` resting orders.
  RestsFull,
  /// The side already holds `caps.max_levels` distinct price levels.
  LevelsFull,
  /// The account already holds `caps.max_account_rests` resting orders.
  AccountFull,
}

impl core::fmt::Display for CapError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::RestsFull => write!( f, "the level is already at its configured rest capacity" ),
      Self::LevelsFull => write!( f, "the side is already at its configured level capacity" ),
      Self::AccountFull => write!( f, "the account is already at its configured resting-order capacity" ),
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

/// Whether an account already holding `current_account_rests` resting orders
/// may rest one more, under `caps`.
///
/// # Errors
///
/// [`CapError::AccountFull`] if `current_account_rests` has already reached
/// `caps.max_account_rests`.
pub const fn cap_check_account( caps : BookCaps, current_account_rests : usize ) -> Result< (), CapError >
{
  if current_account_rests >= caps.max_account_rests
  {
    return Err( CapError::AccountFull );
  }

  Ok( () )
}
