//! Top-N book depth, read directly off [`exchange_book::Book`]'s own
//! priority-ordered slices — never a full walk.
//!
//! Without this crate, a caller wanting "what's the market look like" has to
//! read every resting order on both sides and aggregate by hand. This crate
//! closes hard problem 13 (depth) and feature 17 (`depth_top`).
//!
//! # Why this needs no full walk
//!
//! [`exchange_book::Book::side`] already yields each side in strict priority
//! order — best price first, and every order at one price adjacent
//! (`exchange_book`'s own documented invariant: "Price first, then
//! arrival... Within one price, strictly the earlier arrival"). Reading the
//! top `n` *distinct prices* off that iterator therefore never requires
//! looking past the first `n` price changes, however many orders rest behind
//! them.
//!
//! # `LevelView` aggregates, it doesn't enumerate
//!
//! One [`LevelView`] per distinct price, not per resting order — several
//! orders resting at the same price contribute one row with their summed
//! quantity, matching how a market-depth display is actually read.

use exact_arith::{ Money, Quantity, qty_saturating_add };
use exchange_book::{ Book, Resting };
use exchange_id::InstrumentId;
use exchange_side::Side;

/// Every resting order at one price, aggregated.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct LevelView
{
  /// The price every order aggregated into this level rests at.
  pub price : Money,
  /// Total resting quantity across every order at `price`.
  pub qty : Quantity,
}

/// The top levels on both sides of a book, best-first.
#[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
pub struct Depth
{
  /// Bid levels, best (highest) price first.
  pub bids : Vec< LevelView >,
  /// Ask levels, best (lowest) price first.
  pub asks : Vec< LevelView >,
}

/// Why a depth query was refused.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum DepthError
{
  /// `n` was zero. "The top zero levels" has no sensible answer, unlike a
  /// `n` larger than what rests, which just returns fewer rows.
  BadN,
}

/// The best `n` price levels on each side of `instrument`'s book.
///
/// # Errors
///
/// Returns [`DepthError::BadN`] if `n` is zero. A `n` larger than the
/// number of distinct prices resting on a side is not an error — that side's
/// `Vec` is simply shorter than `n`.
pub fn depth_top( book : &Book, instrument : InstrumentId, n : usize ) -> Result< Depth, DepthError >
{
  if n == 0
  {
    return Err( DepthError::BadN );
  }

  Ok( Depth
  {
    bids : levels_top( book.side( instrument, Side::Buy ), n ),
    asks : levels_top( book.side( instrument, Side::Sell ), n ),
  } )
}

/// Aggregate `resting` (already in priority order) into up to `n` levels.
fn levels_top< 'a >( resting : impl Iterator< Item = &'a Resting >, n : usize ) -> Vec< LevelView >
{
  let mut levels : Vec< LevelView > = Vec::new();

  for r in resting
  {
    if let Some( top ) = levels.last_mut()
      && top.price == r.order.price
    {
      top.qty = qty_saturating_add( top.qty, r.remaining );
      continue;
    }

    if levels.len() == n
    {
      break;
    }
    levels.push( LevelView { price : r.order.price, qty : r.remaining } );
  }

  levels
}
