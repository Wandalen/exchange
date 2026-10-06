//! Which side of the book an order stands on.
//!
//! A root of the dependency tree: no dependency on any other `exchange_*`
//! crate, and no knowledge of price. Without a dedicated type, bid and ask
//! collapse into a bare `bool`, losing the name at every call site.
//!
//! `Side` moved here verbatim from `exchange_types`, which still re-exports
//! it so existing callers are unaffected.
//!
//! # Naming — `Buy`/`Sell`, not `Bid`/`Ask`
//!
//! The source design names the variants `Bid`/`Ask`; the real type has
//! always been `Buy`/`Sell`, because what the rest of the family reasons
//! about is the economic action, not the order-book-display term for it —
//! `exchange_types::obligation` branches on "a buy owes cash, a sell owes
//! the asset," which reads directly off `Buy`/`Sell` and would need a mental
//! translation off `Bid`/`Ask`. Renaming the type would touch every one of
//! the family's five real crates plus workstream 010's consumer for a label
//! change with no behavioural benefit, so it stays. [`side_is_bid`] and
//! [`side_is_ask`] below give the source's own vocabulary a home for callers
//! that want it, without moving it onto the type itself.

/// Which side of the book an order stands on.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub enum Side
{
  /// Bids. Buys currency-for-asset; reserves cash.
  Buy,
  /// Asks. Sells asset-for-currency; reserves the asset.
  Sell,
}

impl Side
{
  /// The side an incoming order matches against.
  #[ must_use ]
  pub const fn opposite( self ) -> Self
  {
    match self
    {
      Self::Buy => Self::Sell,
      Self::Sell => Self::Buy,
    }
  }
}

/// The side an incoming order matches against — a free-function form of
/// [`Side::opposite`], named per the source design's noun-verb convention.
#[ must_use ]
pub const fn side_opposite( side : Side ) -> Side
{
  side.opposite()
}

/// Whether `side` is the bid side (a buy).
#[ must_use ]
pub const fn side_is_bid( side : Side ) -> bool
{
  matches!( side, Side::Buy )
}

/// Whether `side` is the ask side (a sell).
#[ must_use ]
pub const fn side_is_ask( side : Side ) -> bool
{
  matches!( side, Side::Sell )
}
