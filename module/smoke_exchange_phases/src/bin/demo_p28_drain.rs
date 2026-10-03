//! Phase P28 — two producers, each on its own OS thread, draining in one
//! fixed, deterministic combined order regardless of real thread scheduling.
//! Golden: `a=0x... b=0x...` then `ok`.
//!
//! "Two producers" is two rings, each with its own exclusively-owned
//! `Producer`, genuinely raced from two real threads — `ring_handle::Producer`
//! cannot be cloned, so one ring can never have two. The drain side combines
//! both rings' results in a fixed lane order (ring A fully, then ring B)
//! decided here, never by which thread happened to finish publishing first —
//! see `exchange_inbound`'s own module doc and
//! `docs/decisions/001_two_producers_is_two_rings.md` for the full case.

use exchange_id::{ InstrumentId, OrderId };
use exchange_inbound::{ inbound_drain, inbound_ring, InboundCmd };
use std::thread;

const INSTRUMENT : InstrumentId = InstrumentId( 1 );

/// FNV-1a, folded over each command's identifying `OrderId` in drain order.
///
/// Order-sensitive by construction — not the hash-bucket-iteration anti-
/// pattern this family avoids elsewhere (`exchange_snap`'s own documented
/// pitfall): this folds a sequence that is already in a fixed order, so the
/// same sequence always folds to the same value and a differently-ordered
/// sequence folds to a different one.
fn checksum( cmds : &[ InboundCmd ] ) -> u64
{
  let mut hash : u64 = 0xcbf2_9ce4_8422_2325;
  for cmd in cmds
  {
    let id = match cmd
    {
      InboundCmd::Cancel { id, .. } => id.0,
      InboundCmd::Replace { old_id, .. } => old_id.0,
      InboundCmd::Place( resting ) => resting.order.id.0,
    };
    for byte in id.to_le_bytes()
    {
      hash ^= u64::from( byte );
      hash = hash.wrapping_mul( 0x0000_0100_0000_01b3 );
    }
  }
  hash
}

/// Race two producer threads against two rings, then combine deterministically.
fn run_once() -> u64
{
  let mut ring_a = inbound_ring( 64 ).unwrap();
  let mut ring_b = inbound_ring( 64 ).unwrap();
  let mut ends_a = ring_a.ends();
  let mut ends_b = ring_b.ends();
  let ( mut producer_a, mut consumer_a ) = ends_a.split();
  let ( mut producer_b, mut consumer_b ) = ends_b.split();

  thread::scope( | scope |
  {
    scope.spawn( move ||
    {
      for id in 1..=50u64
      {
        producer_a.try_push( InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( id ) } ).unwrap();
      }
    } );
    scope.spawn( move ||
    {
      for id in 101..=150u64
      {
        producer_b.try_push( InboundCmd::Cancel { instrument : INSTRUMENT, id : OrderId( id ) } ).unwrap();
      }
    } );
  } );

  // Fixed lane order, decided here — never by which thread finished first.
  let mut combined = inbound_drain( &mut consumer_a );
  combined.extend( inbound_drain( &mut consumer_b ) );
  checksum( &combined )
}

fn main()
{
  let a = run_once();
  let b = run_once();

  println!( "a=0x{a:x} b=0x{b:x}" );
  assert_eq!( a, b, "same two producers, same fixed combine order — must reproduce despite real thread races" );
  println!( "ok" );
}
