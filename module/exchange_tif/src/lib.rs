//! Time-in-force as an explicit value on the order.
//!
//! A root of the dependency tree: no dependency on any other `exchange_*`
//! crate, and this crate does not itself match — [`tif_rests`] and
//! [`tif_requires_full`] are queries the matching loop consults, not
//! behaviour this crate performs. Without this crate every order can only
//! ever rest forever; there has been no way to say IOC or FOK.
//!
//! `exchange_types`'s own module documentation named this gap directly: "Time-in-Force
//! disposition ... is not implemented." This crate is what closes it —
//! hard problem 20, feature 16.

/// How long a resting order's remainder may live once a match pass is done.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Hash ) ]
pub enum Tif
{
  /// Good-'til-cancelled. A remainder rests until it fills or is cancelled.
  Gtc,
  /// Immediate-or-cancel. Whatever does not fill immediately is withdrawn;
  /// nothing from this order ever rests.
  Ioc,
  /// Fill-or-kill. The order must fill completely or not at all — a partial
  /// outcome is rejected as if nothing had matched.
  Fok,
}

/// Whether an unfilled remainder of `tif` is allowed to rest on the book.
///
/// Only [`Tif::Gtc`] does — [`Tif::Ioc`] withdraws its remainder immediately,
/// and [`Tif::Fok`] never has a partial remainder to rest in the first place.
#[ must_use ]
pub const fn tif_rests( tif : Tif ) -> bool
{
  matches!( tif, Tif::Gtc )
}

/// Whether `tif` accepts only a complete fill, rejecting any partial outcome.
///
/// Only [`Tif::Fok`] does. [`Tif::Gtc`] and [`Tif::Ioc`] both accept a
/// partial fill — they differ only in what happens to what's left.
#[ must_use ]
pub const fn tif_requires_full( tif : Tif ) -> bool
{
  matches!( tif, Tif::Fok )
}
