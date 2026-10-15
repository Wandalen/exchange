//! Wall smoke `smoke_exchange_book` — every stage in one run, against the
//! P30 golden block.
//!
//! Everything this binary drives lives in the crate's library, so the lane
//! is measured like the rest of the crate rather than dropped from the
//! coverage denominator the way a `src/main.rs` is. See
//! [`smoke_exchange_book`] for the scenario and why so little is left here.

fn main()
{
  smoke_exchange_book::run();
}
