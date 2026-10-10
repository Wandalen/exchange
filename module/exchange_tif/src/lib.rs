//! Time-in-force as an explicit value on the order.
//!
//! A root of the dependency tree. This crate does not match; it answers three
//! questions for the code that does. `exchange_match::cross` gates FOK on
//! [`tif_requires_full`] and refuses a post-only order that fails
//! [`tif_takes`]; `exchange_core` and `exchange_inbound` drop an unfilled
//! remainder that fails [`tif_rests`].
//!
//! [`Tif::PostOnly`] is not in the source design — see
//! `docs/decisions/001_post_only_is_a_tif.md`.

/// What happens to an order's unfilled remainder once a match pass is done.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Hash ) ]
pub enum Tif
{
  /// Good-'til-cancelled. The remainder rests until it fills or is cancelled.
  Gtc,
  /// Immediate-or-cancel. Whatever does not fill immediately is withdrawn.
  Ioc,
  /// Fill-or-kill. Fills completely, or is killed whole with the book
  /// unchanged.
  Fok,
  /// Post-only. Rests like [`Tif::Gtc`], but is refused whole if it would
  /// take liquidity on arrival — it only ever trades as the maker.
  PostOnly,
}

/// Whether an unfilled remainder under `tif` may rest on the book.
///
/// [`Tif::Gtc`] and [`Tif::PostOnly`]: [`Tif::Ioc`] withdraws its remainder,
/// and [`Tif::Fok`] never leaves one.
#[ must_use ]
pub const fn tif_rests( tif : Tif ) -> bool
{
  matches!( tif, Tif::Gtc | Tif::PostOnly )
}

/// Whether `tif` refuses a partial fill.
///
/// Only [`Tif::Fok`]. The others accept one and differ only in what happens
/// to the remainder.
#[ must_use ]
pub const fn tif_requires_full( tif : Tif ) -> bool
{
  matches!( tif, Tif::Fok )
}

/// Whether an order under `tif` may take liquidity — trade against an order
/// already resting.
///
/// Every value but [`Tif::PostOnly`].
#[ must_use ]
pub const fn tif_takes( tif : Tif ) -> bool
{
  !matches!( tif, Tif::PostOnly )
}
