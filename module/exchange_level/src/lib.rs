//! One price, FIFO rest — the queue a book keeps at a single price point.
//!
//! Without a dedicated level, there is no price-time priority to speak of:
//! [`Book`](https://docs.rs/exchange_book)'s own priority rule ("price first,
//! then arrival") needs *some* place arrival order lives once two orders
//! share a price. Before this crate existed, that place was implicit — a
//! flat sorted [`Vec`] where same-price orders simply happened to sit next to
//! each other. `Level` makes it explicit: one price, one FIFO queue, nothing
//! else.
//!
//! # Divergence from the source proposal
//!
//! The source names `Level { price, head, len }` / `LevelNode { order, next }`
//! — an intrusive singly-linked list, each node holding a pointer to the
//! next. Rust has no safe way to express "a struct holding a pointer to the
//! next instance of itself" without `unsafe`, `Box`, or `Rc<RefCell<_>>`, and
//! nothing else in this family uses any of the three. `Level` here is a
//! thin wrapper over `Vec<LevelNode>` instead: `level_push` appends (newest
//! arrival, at the back), `level_pop_front` removes index `0` (oldest
//! arrival, at the front). This is not a performance regression —
//! `exchange_book`'s own pre-`exchange_level` code already paid an
//! equivalent `Vec::remove(0)` cost removing the front of its flat
//! per-side vector; moving that same operation one level down changes
//! nothing about its complexity.
//!
//! The proposal's `LevelError { Full, Missing }` is not built. `Full` has no
//! referent at this layer — a capacity ceiling on how many orders may rest is
//! [`exchange_cap`](https://docs.rs/exchange_cap)'s concern (`CapError::RestsFull`),
//! enforced before a node ever reaches a level, not something a level
//! refuses on its own account. `Missing` — removing an id that is not
//! present — is handled the way this same family already handles "not found
//! by id" one crate over: `exchange_book::Book::cancel` returns
//! [`Option`], documented as "a race result rather than a caller error,
//! since the order may have filled or been cancelled already." `level_remove`
//! follows the identical precedent rather than inventing a second vocabulary
//! for the same situation.
//!
//! # Why `nodes` is public
//!
//! The free functions (`level_push`, `level_pop_front`, `level_remove`) are
//! the documented way to keep a level's arrival order correct, but
//! [`Level::nodes`] stays a public field rather than hiding behind an
//! accessor. A book retrofit needs to reduce the *front* node's `remaining`
//! in place — the same partial-fill case `Resting` already handles today —
//! without popping and re-pushing it, which would require a "push to front"
//! operation the FIFO contract should not expose (anything that can insert
//! at the front from outside the crate can also break arrival order from
//! outside the crate). `level.nodes.first_mut()` reaches the same node
//! `Vec::first_mut` already reached in `exchange_book`'s pre-retrofit code,
//! with no new API invented to do it. This mirrors [`LevelNode`]'s own
//! all-public fields, and `Resting`'s before it.

use exact_kind::{ Price, Quantity };
use exchange_id::OrderId;
use exchange_order::Order;
use exchange_seq::Sequence;

/// An order resting at one price, with what is left of it and its arrival
/// position.
///
/// Identical in shape to `exchange_book`'s pre-retrofit `Resting` — this is
/// that type, moved here and renamed to match the level it now lives inside.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct LevelNode
{
  /// The order as submitted. `order.quantity` never changes; `remaining` is
  /// what is left of it.
  pub order : Order,
  /// The unfilled part. Always greater than zero — a resting order with
  /// nothing left is removed rather than kept at zero.
  pub remaining : Quantity,
  /// The position this order claimed on arrival, and its time priority
  /// within this level.
  pub arrival : Sequence,
}

/// Every order resting at one price, in arrival order — index `0` is the
/// oldest, next to be consumed.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct Level
{
  /// The price every node in this level shares. Read-only in practice: a
  /// level's price is its identity and never changes after
  /// [`level_new`].
  pub price : Price,
  /// The resting nodes, oldest first. Public so a book can reduce the front
  /// node's `remaining` in place — see the module doc's "Why `nodes` is
  /// public" section.
  pub nodes : Vec< LevelNode >,
}

/// An empty level at `price`.
#[ must_use ]
pub fn level_new( price : Price ) -> Level
{
  Level { price, nodes : Vec::new() }
}

/// Add `node` as the newest arrival — the back of the queue.
///
/// Does not check `node`'s price against `level.price`, or that its id is
/// not already present: both are a caller contract, the same way
/// `exchange_book::Book::insert` already validates quantity and id *before*
/// ever reaching the per-price placement this function performs.
pub fn level_push( level : &mut Level, node : LevelNode )
{
  level.nodes.push( node );
}

/// Remove and return the oldest arrival — the front of the queue.
///
/// [`None`] if the level is empty.
pub fn level_pop_front( level : &mut Level ) -> Option< LevelNode >
{
  if level.nodes.is_empty()
  {
    None
  }
  else
  {
    Some( level.nodes.remove( 0 ) )
  }
}

/// Remove the node with this id, wherever it sits in arrival order.
///
/// Returns what was removed, or [`None`] if no such node is in this level —
/// a race result, not a caller error; see the module doc's "Divergence"
/// section for why this is `Option` rather than the proposal's
/// `LevelError::Missing`.
pub fn level_remove( level : &mut Level, id : OrderId ) -> Option< LevelNode >
{
  let at = level.nodes.iter().position( | node | node.order.id == id )?;
  Some( level.nodes.remove( at ) )
}

/// How many nodes rest in this level.
#[ must_use ]
pub fn level_len( level : &Level ) -> usize
{
  level.nodes.len()
}

/// The sum of every node's `remaining` in this level.
///
/// # Panics
///
/// If the sum overflows `Quantity`'s representable range. No real caller
/// approaches this: it would require more total quantity resting at a
/// single price than the backing currency can express across every order
/// ever placed. Left as an honest `expect` rather than threading a `Result`
/// through a function no test or caller has ever needed to fail.
#[ must_use ]
pub fn level_qty_sum( level : &Level ) -> Quantity
{
  level.nodes.iter().try_fold( Quantity::ZERO, | sum, node | sum.checked_add( node.remaining ) )
    .expect( "no level in this family ever rests enough total quantity to overflow Quantity" )
}

/// Whether this level has no nodes left — the signal a book uses to drop the
/// level itself rather than keep an empty price around.
#[ must_use ]
pub fn level_empty_is( level : &Level ) -> bool
{
  level.nodes.is_empty()
}
