//! An `OrderId` seen once per book.
//!
//! Depends on `exchange_id` alone and knows nothing about a book, a price, or
//! a side. A repeat gets its own refusal, [`IdemError::Duplicate`], checkable
//! before the id reaches `exchange_book::Book::insert` — which refuses a
//! duplicate too, but with the same `false` it returns for a zero remainder.
//! Closes hard problem 15 and feature 19.
//!
//! # Any key, `OrderId` by default
//!
//! Where the exchange assigns `OrderId` itself, only the submitter's own id
//! marks a retry — key the set by `( AccountId, ClientOrderId )` there.
//!
//! # Membership, not priority
//!
//! Backed by a [`std::collections::HashSet`]. The family's "no hash iteration"
//! rule (see `exchange_book`) keeps hash order out of priority decisions;
//! `IdSet` is never iterated, so its order is never observed.

use core::hash::Hash;
use std::collections::HashSet;

use exchange_id::OrderId;

/// The set of keys already seen — by default, the `OrderId`s seen by one book.
#[ derive( Debug, Clone ) ]
pub struct IdSet< K = OrderId >
{
  seen : HashSet< K >,
}

// Not derived: a derived `Default` would demand `K : Default`, which no id
// type needs.
impl< K > Default for IdSet< K >
{
  fn default() -> Self
  {
    Self { seen : HashSet::new() }
  }
}

impl< K > IdSet< K >
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
  /// This key was already seen by this set.
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
pub fn idem_seen< K : Hash + Eq >( set : &IdSet< K >, id : K ) -> bool
{
  set.seen.contains( &id )
}

/// Record `id` as seen, refusing a repeat.
///
/// # Errors
///
/// [`IdemError::Duplicate`] if `id` was already in `set` — `set` is left
/// unchanged by a refused insert.
pub fn idem_insert< K : Hash + Eq >( set : &mut IdSet< K >, id : K ) -> Result< (), IdemError >
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
pub fn idem_remove< K : Hash + Eq >( set : &mut IdSet< K >, id : K ) -> bool
{
  set.seen.remove( &id )
}
