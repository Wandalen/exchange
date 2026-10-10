//! One price, FIFO rest — the queue a book keeps at a single price point.
//!
//! `exchange_book` keeps one [`Level`] per price per side. Within a level,
//! index `0` is the oldest arrival and the next to trade.
//!
//! # Not the proposal's intrusive list
//!
//! The source design names `Level { price, head, len }` /
//! `LevelNode { order, next }`. Safe Rust has no self-referential node chain
//! without `Box` or `Rc`, so [`Level`] keeps its nodes in a [`VecDeque`]:
//! taking the front is O(1), so sweeping a level is linear in its length.
//! `LevelError` is not built — see `docs/decisions/001_no_level_error.md`.
//!
//! # `nodes` is public
//!
//! A partial fill reduces the front node's `remaining` in place. The
//! alternative — pop, then push back to the front — needs an operation that
//! would let any caller break arrival order.

use std::collections::VecDeque;

use exact_arith::{ KindError, Price, Quantity };
use exchange_id::OrderId;
use exchange_order::Order;
use exchange_seq::Sequence;

/// An order resting at one price, with what is left of it and its arrival
/// position. `exchange_book::Resting` is an alias of this type.
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
  /// The price every node in this level shares — its identity, never
  /// changed after [`level_new`].
  pub price : Price,
  /// The resting nodes, oldest first. Public so a book can reduce the front
  /// node's `remaining` in place — see the module doc.
  pub nodes : VecDeque< LevelNode >,
}

/// An empty level at `price`.
#[ must_use ]
pub fn level_new( price : Price ) -> Level
{
  Level { price, nodes : VecDeque::new() }
}

/// Add `node` as the newest arrival — the back of the queue.
///
/// Neither `node`'s price nor its id is checked; `exchange_book::Book::insert`
/// checks both before a node reaches a level.
pub fn level_push( level : &mut Level, node : LevelNode )
{
  level.nodes.push_back( node );
}

/// Remove and return the oldest arrival — the front of the queue.
///
/// [`None`] if the level is empty.
pub fn level_pop_front( level : &mut Level ) -> Option< LevelNode >
{
  level.nodes.pop_front()
}

/// Remove the node with this id, wherever it sits in arrival order.
///
/// [`None`] if it is not here — a race result, not an error; see
/// `docs/decisions/001_no_level_error.md`.
pub fn level_remove( level : &mut Level, id : OrderId ) -> Option< LevelNode >
{
  let at = level.nodes.iter().position( | node | node.order.id == id )?;
  level.nodes.remove( at )
}

/// How many nodes rest in this level.
#[ must_use ]
pub fn level_len( level : &Level ) -> usize
{
  level.nodes.len()
}

/// The sum of every node's `remaining` in this level.
///
/// # Errors
///
/// [`KindError`] if the sum passes `Quantity`'s ceiling — two accounts each
/// resting close to it at one price are enough.
pub fn level_qty_sum( level : &Level ) -> Result< Quantity, KindError >
{
  level.nodes.iter().try_fold( Quantity::ZERO, | sum, node | sum.checked_add( node.remaining ) )
}

/// Whether this level has no nodes left — the signal a book uses to drop the
/// level rather than keep an empty price.
#[ must_use ]
pub fn level_empty_is( level : &Level ) -> bool
{
  level.nodes.is_empty()
}
