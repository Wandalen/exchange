//! Inbound commands over a ring: a producer publishes [`InboundCmd`]s, one
//! consumer drains them in order and applies them to a [`Book`].
//!
//! `exchange_core::Exchange::exchange_step` drains these rings and applies
//! each command itself, with escrow, caps and halt. [`inbound_apply`] is the
//! book-level apply, for a caller without the facade.
//!
//! # One producer per ring
//!
//! `ring_handle::Producer` cannot be cloned, and this crate uses the `ring_*`
//! export crates only, so a ring has exactly one producer. Many producers are
//! many rings, each raced from its own thread and drained in a lane order the
//! caller fixes ahead of time, never in the order the threads finished
//! (`docs/decisions/001_two_producers_is_two_rings.md`).
//!
//! # Overflow is a reject
//!
//! `RingConfig::new` defaults to `OverflowPolicy::DropNewest`, under which a
//! full ring's `try_push` returns `Ok` and discards the record.
//! [`inbound_ring`] always sets [`OverflowPolicy::Fail`], so
//! [`inbound_overflow_reject`] hands back a command it could not publish.
//!
//! # Applying a command
//!
//! A [`InboundCmd::Place`] is a new order: `exchange_match::cross` crosses it
//! first, and only a remainder its time in force lets rest goes to
//! `exchange_rest::rest_place`. Cancel and replace act on a resting order and
//! go to `exchange_rest`. Arrivals are the producer's; nothing here
//! sequences.
//!
//! [`Claims`] carries idempotency between calls, as `exchange_core` keeps
//! it. A `Place` naming a claimed order id, or a client id its account already
//! used, is refused before it crosses. An order id is claimed when its order
//! rests and released when a cancel or replace withdraws it; a client id is
//! claimed for good.

use exact_arith::Quantity;
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, ClientOrderId, InstrumentId, OrderId };
use exchange_idem::{ idem_insert, idem_remove, idem_seen };
use exchange_match::{ cross, Crossing, MatchError, SelfMatchPolicy };
use exchange_rest::{ rest_cancel, rest_place, rest_replace, RestReplaceError };
use exchange_tif::tif_rests;
use ring_factory::Factory;
use ring_types::OverflowPolicy;

pub use exchange_idem::{ IdemError, IdSet };
pub use ring_factory::{ BuildError, RingConfig };
pub use ring_handle::{ Consumer, Drain, Ends, Producer, Split };

/// One command from a producer to the book.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum InboundCmd
{
  /// A new order: its `remaining` the whole quantity, its `arrival` as its
  /// producer stamped it.
  Place( Resting ),
  /// Withdraw `id`, resting on `instrument`'s book, whichever side.
  Cancel
  {
    /// Which instrument's book `id` rests on.
    instrument : InstrumentId,
    /// The resting order to withdraw.
    id : OrderId,
  },
  /// Replace `old_id` with `new_resting` in one step —
  /// [`exchange_rest::rest_replace`].
  Replace
  {
    /// Which instrument's book `old_id` rests on.
    instrument : InstrumentId,
    /// The resting order being replaced.
    old_id : OrderId,
    /// What replaces it, if `old_id` is found.
    new_resting : Resting,
  },
}

/// What applying one [`InboundCmd`] produced.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum InboundOutcome
{
  /// [`InboundCmd::Place`] was crossed against the book.
  Crossed( Crossing ),
  /// [`InboundCmd::Cancel`] ran; [`None`] if nothing was resting under that id.
  Cancelled( Option< Resting > ),
  /// [`InboundCmd::Replace`] ran; see [`RestReplaceError`] for the failure shapes.
  Replaced( Result< Resting, RestReplaceError > ),
}

/// Why [`inbound_apply`] could not apply a drained command.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum InboundApplyError
{
  /// [`exchange_match::cross`] refused: in practice
  /// [`MatchError::PostOnlyWouldTake`], with `book` untouched.
  Match( MatchError ),
  /// A [`InboundCmd::Place`] named an order id [`Claims`] holds — a retry,
  /// not a new order.
  Idem( IdemError ),
  /// A [`InboundCmd::Place`] carried a client id its account already placed
  /// an order under — a retry, not a new order.
  DuplicateClientId,
  /// A [`InboundCmd::Place`] for a quantity of zero.
  ZeroQuantity,
  /// A [`InboundCmd::Place`] whose `remaining` is not its quantity: a new
  /// order has none of it filled.
  NotNew,
}

impl core::fmt::Display for InboundApplyError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::Match( error ) => write!( f, "matching failed: {error}" ),
      Self::Idem( error ) => write!( f, "place refused: {error}" ),
      Self::DuplicateClientId => write!( f, "place refused: this account already placed an order under this client id" ),
      Self::ZeroQuantity => write!( f, "place refused: the quantity is zero" ),
      Self::NotNew => write!( f, "place refused: a new order's remaining must be its whole quantity" ),
    }
  }
}

impl core::error::Error for InboundApplyError
{
  fn source( &self ) -> Option< &( dyn core::error::Error + 'static ) >
  {
    match self
    {
      Self::Match( error ) => Some( error ),
      Self::Idem( error ) => Some( error ),
      Self::DuplicateClientId | Self::ZeroQuantity | Self::NotNew => None,
    }
  }
}

impl From< MatchError > for InboundApplyError
{
  fn from( error : MatchError ) -> Self
  {
    Self::Match( error )
  }
}

impl From< IdemError > for InboundApplyError
{
  fn from( error : IdemError ) -> Self
  {
    Self::Idem( error )
  }
}

/// What [`inbound_apply`] has claimed: an order id while its order rests, and
/// every `( AccountId, ClientOrderId )` it accepted, for good.
#[ derive( Debug, Clone, Default ) ]
pub struct Claims
{
  orders : IdSet,
  clients : IdSet< ( AccountId, ClientOrderId ) >,
}

impl Claims
{
  /// Nothing claimed yet.
  #[ must_use ]
  pub fn new() -> Self
  {
    Self::default()
  }
}

/// Build one ring of `capacity` command slots, one producer, overflow
/// refused — see the module doc.
///
/// # Errors
///
/// [`BuildError`] on a `capacity` of zero or not a power of two. Every
/// backend accepts [`OverflowPolicy::Fail`].
pub fn inbound_ring( capacity : usize ) -> Result< Split< InboundCmd >, BuildError >
{
  let cfg = RingConfig::new( capacity ).map_err( BuildError::Unsupported )?.with_overflow( OverflowPolicy::Fail );
  Factory.build::< InboundCmd >( cfg )
}

/// Publish as many of `cmds` as the ring takes, and return how many —
/// [`Producer::try_push_batch`]. A partial batch is not a failure.
//
// Fix(flush_accepts_as_many_as_fit_and_reports_the_count):
// Root cause: written against a `usize`-returning `try_push_batch`, which
// returns `Result< usize, ( usize, T ) >`, so it never compiled.
// Pitfall: a thin forward still has to match what it forwards to.
pub fn inbound_flush( producer : &mut Producer< '_, InboundCmd >, cmds : impl IntoIterator< Item = InboundCmd > ) -> usize
{
  let mut cmds = cmds.into_iter();
  match producer.try_push_batch( &mut cmds )
  {
    Ok( n ) | Err( ( n, _ ) ) => n,
  }
}

/// Publish one command — [`Producer::try_push`].
///
/// # Errors
///
/// Hands `cmd` back, unpublished, when the ring is full. Only a ring built
/// with [`OverflowPolicy::Fail`], as [`inbound_ring`] builds them, says so.
pub fn inbound_overflow_reject( producer : &mut Producer< '_, InboundCmd >, cmd : InboundCmd ) -> Result< (), InboundCmd >
{
  producer.try_push( cmd )
}

/// Every command published before this call, in order — [`Consumer::drain`].
/// Bounded at the call, so it returns while a producer still publishes.
pub fn inbound_drain( consumer : &mut Consumer< '_, InboundCmd > ) -> Vec< InboundCmd >
{
  consumer.drain().collect()
}

/// Apply one drained [`InboundCmd`] to `book` — see the module doc's
/// "Applying a command". `policy` resolves a self-match, as in
/// `exchange_match::cross`.
///
/// # Errors
///
/// Each with `book` and `claims` untouched: [`InboundApplyError::Match`] when
/// `cross` refuses, in practice a post-only order that would take;
/// [`InboundApplyError::ZeroQuantity`] for a `Place` for nothing;
/// [`InboundApplyError::NotNew`] for one whose `remaining` is not its
/// quantity; [`InboundApplyError::Idem`] for one naming a claimed order id;
/// [`InboundApplyError::DuplicateClientId`] for one carrying a client id its
/// account already used.
pub fn inbound_apply( book : &mut Book, claims : &mut Claims, policy : SelfMatchPolicy, cmd : InboundCmd ) -> Result< InboundOutcome, InboundApplyError >
{
  match cmd
  {
    InboundCmd::Place( resting ) =>
    {
      // Fix(a_duplicate_place_id_is_refused_before_it_trades):
      // Root cause: a repeated id was checked only where its remainder would
      // rest, after `cross` had traded it, so the caller got `Err` and lost
      // the trades.
      // Pitfall: refuse before the first mutation, not before the last one.
      let order = resting.order;
      // Fix(a_zero_quantity_place_is_refused_and_claims_nothing):
      // Root cause: nothing read the quantity, so a place for nothing crossed
      // nothing, came back `Ok`, and claimed its client id for good.
      // Pitfall: `exchange_core` refuses it before anything else; a second
      // way onto the book needs the same first check.
      if order.quantity == Quantity::ZERO
      {
        return Err( InboundApplyError::ZeroQuantity );
      }
      // Fix(a_place_whose_remaining_is_not_its_quantity_is_refused):
      // Root cause: `cross` trades `order.quantity` and the remainder rests
      // from what it leaves, so the `remaining` a place carried went unread:
      // 3 left of 10 traded all 10.
      // Pitfall: a field a command carries and nothing reads is one its
      // producer still believes in.
      if resting.remaining != order.quantity
      {
        return Err( InboundApplyError::NotNew );
      }
      if idem_seen( &claims.orders, order.id )
      {
        return Err( InboundApplyError::Idem( IdemError::Duplicate ) );
      }
      let client = order.client.map( | client | ( order.account, client ) );
      if client.is_some_and( | key | idem_seen( &claims.clients, key ) )
      {
        return Err( InboundApplyError::DuplicateClientId );
      }
      let crossing = cross( book, &order, policy )?;
      if let Some( key ) = client
      {
        idem_insert( &mut claims.clients, key ).expect( "checked unseen before crossing" );
      }

      // Fix(place_does_not_rest_an_order_the_self_match_policy_cancelled):
      // Root cause: `crossing.remaining` keeps a self-match-cancelled
      // remainder too, and only it was read, so a cancelled incoming order
      // rested anyway.
      // Pitfall: `remaining > 0` means not filled, not free to rest; only
      // `crossing.cancelled` tells the two apart.
      let incoming_cancelled = crossing.cancelled.iter().any( | c | c.order == resting.order.id );

      if !incoming_cancelled && !crossing.is_complete() && tif_rests( resting.order.tif )
      {
        // Fix(duplicate_place_id_is_refused_not_silently_dropped):
        // Root cause: `rest_place` refusing a repeated id was checked by a
        // `debug_assert!` only, so in release the remainder neither rested
        // nor was reported.
        // Pitfall: a public function cannot assume the fresh ids its one
        // in-repo caller happens to send.
        idem_insert( &mut claims.orders, order.id ).expect( "checked unseen before crossing" );
        let remainder = Resting { remaining : crossing.remaining, ..resting };
        let placed = rest_place( book, remainder );
        debug_assert!( placed, "idem_insert above already refused a repeat; a freshly-claimed id cannot also collide in Book::insert" );
      }

      Ok( InboundOutcome::Crossed( crossing ) )
    },
    InboundCmd::Cancel { instrument, id } =>
    {
      let cancelled = rest_cancel( book, instrument, id );
      if cancelled.is_some()
      {
        idem_remove( &mut claims.orders, id );
      }
      Ok( InboundOutcome::Cancelled( cancelled ) )
    },
    InboundCmd::Replace { instrument, old_id, new_resting } =>
    {
      let new_id = new_resting.order.id;
      let result = rest_replace( book, instrument, old_id, new_resting );
      if result.is_ok()
      {
        idem_remove( &mut claims.orders, old_id );
        idem_insert( &mut claims.orders, new_id )
          .expect( "rest_replace's own underlying Book::insert already proved this id free" );
      }
      Ok( InboundOutcome::Replaced( result ) )
    },
  }
}
