//! Time-in-force as an explicit value on the order.
//!
//! A root of the dependency tree. This crate does not match; it answers two
//! questions for the code that does. `exchange_match::cross` gates FOK on
//! [`tif_requires_full`]; `exchange_core` and `exchange_inbound` drop an
//! unfilled remainder that fails [`tif_rests`].

/// What happens to an order's unfilled remainder once a match pass is done.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Hash ) ]
pub enum Tif
{
  /// Good-'til-cancelled. The remainder rests until it fills or is cancelled.
  Gtc,
  /// Immediate-or-cancel. Whatever does not fill immediately is withdrawn.
  Ioc,
  /// Fill-or-kill. Fills completely, or is rejected with the book unchanged.
  Fok,
}

/// Whether an unfilled remainder under `tif` may rest on the book.
///
/// Only [`Tif::Gtc`]: [`Tif::Ioc`] withdraws its remainder, and [`Tif::Fok`]
/// never leaves one.
#[ must_use ]
pub const fn tif_rests( tif : Tif ) -> bool
{
  matches!( tif, Tif::Gtc )
}

/// Whether `tif` refuses a partial fill.
///
/// Only [`Tif::Fok`]. [`Tif::Gtc`] and [`Tif::Ioc`] both accept one and differ
/// only in what happens to the remainder.
#[ must_use ]
pub const fn tif_requires_full( tif : Tif ) -> bool
{
  matches!( tif, Tif::Fok )
}
