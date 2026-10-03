//! An `OrderId` seen once per book.
//!
//! A root of the dependency tree: its only dependency is `exchange_id`, and
//! it knows nothing about a book, a price, or a side — it answers exactly
//! one question, "has this id already been seen," and nothing else.
//!
//! # Genuinely new
//!
//! Before this crate, idempotency was a side effect rather than a decision:
//! `exchange_book::Book::insert` (`module/exchange_book/src/lib.rs`) already
//! refuses a second resting order sharing an `OrderId`, but silently, by
//! returning `false` for the same reason it refuses a zero-remaining insert —
//! a caller cannot tell "this id already rests" from "this quantity was
//! already zero" without a dedicated reason code. This crate closes hard
//! problem 15 (idempotent order ids) and feature 19 (unique `OrderId` per
//! book) by giving that one case its own name, [`IdemError::Duplicate`],
//! checkable *before* an id ever reaches `Book::insert` at all.
//!
//! # Membership, not priority
//!
//! [`IdSet`] stores its seen ids in a [`std::collections::HashSet`]. This
//! does not conflict with the family's own "no hash iteration anywhere" rule
//! (see `exchange_book`'s module doc) — that rule guards against hash
//! *iteration order* leaking into a priority decision. `IdSet` is never
//! iterated; every operation here is a single-key membership test or
//! mutation, for which a hash set's own order is simply never observed.

use exchange_id::OrderId;

/// The set of `OrderId`s already seen by one book.
#[ derive( Debug, Clone, Default ) ]
pub struct IdSet
{
  seen : std::collections::HashSet< OrderId >,
}

impl IdSet
{
  /// An empty set — nothing seen yet.
  #[ must_use ]
  pub fn new() -> Self
  {
    Self::default()
  }
}

/// Why an id could not be inserted.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum IdemError
{
  /// This `OrderId` was already seen by this set.
  Duplicate,
}

impl core::fmt::Display for IdemError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::Duplicate => write!( f, "this order id was already seen" ),
    }
  }
}

impl core::error::Error for IdemError {}

/// Whether `id` has already been seen by `set`.
///
/// A read-only check — unlike [`idem_insert`], calling this never changes
/// `set`, so a caller can test before committing to anything else a failed
/// insert would need to unwind.
#[ must_use ]
pub fn idem_seen( set : &IdSet, id : OrderId ) -> bool
{
  set.seen.contains( &id )
}

/// Record `id` as seen, refusing a repeat.
///
/// # Errors
///
/// [`IdemError::Duplicate`] if `id` was already in `set` — `set` is left
/// unchanged by a refused insert.
pub fn idem_insert( set : &mut IdSet, id : OrderId ) -> Result< (), IdemError >
{
  if set.seen.insert( id )
  {
    Ok( () )
  }
  else
  {
    Err( IdemError::Duplicate )
  }
}

/// Forget `id`, so a future resubmission under the same id is accepted again.
///
/// Returns whether `id` was present to forget — a caller that expected it to
/// be there (releasing a cancelled order's id, say) can tell a genuine
/// "already gone" apart from its own bookkeeping error.
pub fn idem_remove( set : &mut IdSet, id : OrderId ) -> bool
{
  set.seen.remove( &id )
}
