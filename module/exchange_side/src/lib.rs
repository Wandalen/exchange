//! Which side of the book an order stands on.
//!
//! A root of the dependency tree, with no knowledge of price. A dedicated
//! type keeps bid and ask from collapsing into a bare `bool`.
//!
//! The two side-dependent price rules live here once, generic over the price
//! type: [`side_ahead`] for book priority, [`side_accepts`] for a limit.
//!
//! # Naming — `Buy`/`Sell`, not `Bid`/`Ask`
//!
//! The source design names the variants `Bid`/`Ask`. The family reasons
//! about the economic action — `exchange_types::obligation` reads "a buy owes
//! cash, a sell owes the asset" straight off `Buy`/`Sell` — so the variants
//! keep those names, and [`side_is_bid`]/[`side_is_ask`] cover the source's
//! vocabulary. See `docs/decisions/001_buy_sell_not_bid_ask.md`.

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

/// Free-function form of [`Side::opposite`], under the source design's name.
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

/// Whether price `a` ranks ahead of `b` on `side`'s book — higher for bids,
/// lower for asks.
#[ must_use ]
pub fn side_ahead< P : PartialOrd >( side : Side, a : P, b : P ) -> bool
{
  match side
  {
    Side::Buy => a > b,
    Side::Sell => a < b,
  }
}

/// Whether an order on `side` limited at `limit` accepts a trade at `price` —
/// at or below the limit for a buy, at or above for a sell. Equality accepts.
#[ must_use ]
pub fn side_accepts< P : PartialOrd >( side : Side, limit : P, price : P ) -> bool
{
  match side
  {
    Side::Buy => price <= limit,
    Side::Sell => price >= limit,
  }
}
