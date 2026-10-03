//! Smoke lane `smoke_exchange_core` — one order crossing the whole
//! family, and one that must not cross at all.
//!
//! Everything this binary drives lives in the crate's library, so the lane is
//! measured like the rest of the crate rather than dropped from the
//! coverage denominator the way a `src/main.rs` is. See [`smoke_exchange_core`]
//! for the three arms and why so little is left here.

fn main()
{
  smoke_exchange_core::run();
}
