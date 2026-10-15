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
//! (`docs/decisions/001_two_producers_is_two_rings.md`). The combined order
//! repeats from run to run only if each lane is drained once its producer has
//! stopped: [`inbound_drain`] takes what was published before the call, so a
//! live lane is cut wherever its producer happens to be.
//!
//! # Overflow is a reject
//!
//! `RingConfig::new` defaults to `OverflowPolicy::DropNewest`, under which a
//! full ring's `try_push` returns `Ok` and discards the record.
//! [`inbound_ring`] always sets [`OverflowPolicy::Fail`], so
//! [`inbound_overflow_reject`] hands back a command it could not publish.
//! For a batch, [`inbound_flush`]'s count short of `cmds.len()` is the same
//! reject, and it is `#[must_use]`.
//!
//! Only the producer learns of the reject. A command the ring refused never
//! reaches the consumer, so `exchange_core`'s event log has no record of it;
//! telling the sender is the producer's job.
//!
//! # Applying a command
//!
//! A [`InboundCmd::Place`] is a new order: `exchange_match::cross` crosses it
//! first, and only a remainder its time in force lets rest goes to
//! `exchange_rest::rest_place`. Cancel and replace act on a resting order and
//! go to `exchange_rest`. Arrivals are the producer's; nothing here
//! sequences.
//!
//! A `Place` naming an order id that rests, or a client id its account
//! already used, is refused before it crosses. The book says which order ids
//! rest: once an order has left it, however it left, its id is free again.
//! [`Claims`] keeps the client ids, for good, as `exchange_core` keeps them;
//! a retry of an order that filled is known by its client id.

use exact_arith::Quantity;
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, ClientOrderId, InstrumentId, OrderId };
use exchange_idem::{ idem_insert, idem_seen, IdSet };
use exchange_match::{ cross, Crossing, MatchError, SelfMatchPolicy };
use exchange_rest::{ rest_cancel, rest_place, rest_replace, RestReplaceError };
use exchange_tif::tif_rests;
use ring_factory::Factory;
use ring_types::OverflowPolicy;

pub use ring_factory::{ BuildError, RingConfig };
pub use ring_handle::{ Consumer, Drain, Ends, Producer, Split };

/// One command from a producer to the book.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum InboundCmd
{
  /// A new order: its `remaining` the whole quantity, its `arrival` as its
  /// producer stamped it.
  Place( Resting ),
  /// Withdraw `account`'s order named by `order`, resting on
  /// `instrument`'s book, whichever side.
  Cancel
  {
    /// Which instrument's book the order rests on.
    instrument : InstrumentId,
    /// The account asking; only its own order is withdrawn.
    account : AccountId,
    /// The resting order to withdraw.
    order : OrderRef,
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

/// Which order a [`InboundCmd::Cancel`] names: the id the exchange assigned,
/// or the client id the account placed it under. A ring producer never sees
/// the id the exchange assigns, so the client id is how it names its own
/// order.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum OrderRef
{
  /// The id the exchange assigned.
  Id( OrderId ),
  /// The client id the account placed the order under.
  Client( ClientOrderId ),
}

impl OrderRef
{
  /// The order this names, resting on `instrument`'s book under `account`.
  ///
  /// [`None`] when no such order rests: never placed, already gone, on
  /// another instrument, or another account's. All four are the same answer,
  /// so a caller cannot learn which ids another account has live. A client
  /// id names at most one order, because an account never reuses one.
  #[ must_use ]
  pub fn resting_in( self, book : &Book, instrument : InstrumentId, account : AccountId ) -> Option< &Resting >
  {
    book.iter().find( | resting |
    {
      let order = resting.order;
      order.instrument == instrument && order.account == account && match self
      {
        Self::Id( id ) => order.id == id,
        Self::Client( client ) => order.client == Some( client ),
      }
    } )
  }
}

impl From< OrderId > for OrderRef
{
  fn from( id : OrderId ) -> Self
  {
    Self::Id( id )
  }
}

impl From< ClientOrderId > for OrderRef
{
  fn from( client : ClientOrderId ) -> Self
  {
    Self::Client( client )
  }
}

impl core::fmt::Display for OrderRef
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::Id( id ) => write!( f, "order {}", id.0 ),
      Self::Client( client ) => write!( f, "client order {}", client.0 ),
    }
  }
}

/// What applying one [`InboundCmd`] produced.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum InboundOutcome
{
  /// [`InboundCmd::Place`] was crossed against the book.
  Crossed( Crossing ),
  /// [`InboundCmd::Cancel`] ran; [`None`] if the account had no such order
  /// resting on that instrument ([`OrderRef::resting_in`]).
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
  /// A [`InboundCmd::Place`] named an order id that rests on the book.
  OrderIdRests,
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
      Self::OrderIdRests => write!( f, "place refused: an order under this id rests on the book" ),
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
      Self::OrderIdRests | Self::DuplicateClientId | Self::ZeroQuantity | Self::NotNew => None,
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

/// Every `( AccountId, ClientOrderId )` [`inbound_apply`] has accepted, for
/// good. Which order ids are taken, the book says.
#[ derive( Debug, Clone, Default ) ]
pub struct Claims
{
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
/// [`BuildError::Unsupported`], carrying the ring's own error, on a
/// `capacity` of zero or not a power of two. Every backend accepts
/// [`OverflowPolicy::Fail`].
pub fn inbound_ring( capacity : usize ) -> Result< Split< InboundCmd >, BuildError >
{
  let cfg = RingConfig::new( capacity ).map_err( BuildError::Unsupported )?.with_overflow( OverflowPolicy::Fail );
  Factory.build::< InboundCmd >( cfg )
}

/// Publish the front of `cmds`, as much as the ring takes, and return how
/// many — [`Producer::try_push_batch`]. A partial batch is not a failure:
/// `cmds[ n.. ]` is still the caller's to publish later.
///
/// ```
/// #![ deny( unused_must_use ) ]
/// use exchange_id::{ AccountId, InstrumentId, OrderId };
/// use exchange_inbound::{ inbound_flush, inbound_ring, InboundCmd, OrderRef };
///
/// let mut split = inbound_ring( 2 ).unwrap();
/// let mut ends = split.ends();
/// let ( mut producer, _consumer ) = ends.split();
/// let cancel = | id | InboundCmd::Cancel { instrument : InstrumentId( 1 ), account : AccountId( 1 ), order : OrderRef::Id( OrderId( id ) ) };
/// let cmds = [ cancel( 1 ), cancel( 2 ), cancel( 3 ) ];
///
/// let sent = inbound_flush( &mut producer, &cmds );
/// assert_eq!( sent, 2 ); // `cmds[ 2.. ]` is still the caller's
/// ```
///
/// Ignoring the count loses `cmds[ n.. ]` without a trace, so it does not
/// compile under `deny( unused_must_use )`:
///
/// ```compile_fail
/// #![ deny( unused_must_use ) ]
/// use exchange_id::{ AccountId, InstrumentId, OrderId };
/// use exchange_inbound::{ inbound_flush, inbound_ring, InboundCmd, OrderRef };
///
/// let mut split = inbound_ring( 2 ).unwrap();
/// let mut ends = split.ends();
/// let ( mut producer, _consumer ) = ends.split();
/// let cancel = | id | InboundCmd::Cancel { instrument : InstrumentId( 1 ), account : AccountId( 1 ), order : OrderRef::Id( OrderId( id ) ) };
/// let cmds = [ cancel( 1 ), cancel( 2 ), cancel( 3 ) ];
///
/// inbound_flush( &mut producer, &cmds );
/// ```
//
// Fix(flush_accepts_as_many_as_fit_and_reports_the_count):
// Root cause: written against a `usize`-returning `try_push_batch`, which
// returns `Result< usize, ( usize, T ) >`, so it never compiled.
// Pitfall: a thin forward still has to match what it forwards to.
//
// Fix(a_flush_leaves_what_did_not_fit_with_the_caller):
// Root cause: taking any iterator, the flush pulled one command past what
// the ring took and dropped it with the batch's error, so a caller resuming
// from the same iterator lost it.
// Pitfall: a partial batch is the normal case, so its cut must lose nothing.
#[ must_use = "commands past the returned count were not published; drop the count and they are lost unseen" ]
pub fn inbound_flush( producer : &mut Producer< '_, InboundCmd >, cmds : &[ InboundCmd ] ) -> usize
{
  match producer.try_push_batch( &mut cmds.iter().copied() )
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
/// quantity; [`InboundApplyError::OrderIdRests`] for one naming a resting
/// order id; [`InboundApplyError::DuplicateClientId`] for one carrying a
/// client id its account already used.
pub fn inbound_apply( book : &mut Book, claims : &mut Claims, policy : SelfMatchPolicy, cmd : InboundCmd ) -> Result< InboundOutcome, InboundApplyError >
{
  match cmd
  {
    InboundCmd::Place( resting ) =>
    {
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
      // Fix(a_duplicate_place_id_is_refused_before_it_trades):
      // Root cause: a repeated id was checked only where its remainder would
      // rest, after `cross` had traded it, so the caller got `Err` and lost
      // the trades.
      // Pitfall: refuse before the first mutation, not before the last one.
      //
      // Fix(a_replace_may_take_the_id_of_an_order_that_filled):
      // Root cause: order ids were claimed in a set of their own, released by
      // cancel and replace only. An order that filled or was withdrawn by
      // self-match kept its claim, so its id was refused for good, and a
      // replace onto it panicked on the claim after changing the book. An id
      // resting on a book `claims` never saw was not refused at all.
      // Pitfall: a second record of what the book holds drifts from it;
      // ask the book. With the quantity checked above, `insertable` can only
      // refuse the id.
      if !book.insertable( &resting, None )
      {
        return Err( InboundApplyError::OrderIdRests );
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
        let remainder = Resting { remaining : crossing.remaining, ..resting };
        let placed = rest_place( book, remainder );
        assert!( placed, "insertable() held before crossing, and cross seats nothing" );
      }

      Ok( InboundOutcome::Crossed( crossing ) )
    },
    InboundCmd::Cancel { instrument, account, order } =>
    {
      let id = order.resting_in( book, instrument, account ).map( | resting | resting.order.id );
      Ok( InboundOutcome::Cancelled( id.and_then( | id | rest_cancel( book, instrument, id ) ) ) )
    },
    InboundCmd::Replace { instrument, old_id, new_resting } =>
    {
      Ok( InboundOutcome::Replaced( rest_replace( book, instrument, old_id, new_resting ) ) )
    },
  }
}
