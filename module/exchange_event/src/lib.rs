//! The owned drain point for the event stream: workstream 010 takes fills,
//! rejects and cancel-acks out of it, rather than 002 writing wallets.
//! Closes hard problem 12 and feature 26.
//!
//! `exchange_core::Exchange` stores its events in a `Vec< Event >`; `events()`
//! lends them, and `event_drain()` calls [`event_drain`] to take them. The
//! functions take that `Vec` itself: there is no `EventDrain` wrapper and no
//! `EventError { Full }`, since nothing bounds the stream — see
//! `docs/decisions/`. [`Event`]/[`EventKind`] are `exchange_fill`'s,
//! re-exported.

pub use exchange_fill::{ Event, EventKind };

/// Append one event to `events`.
pub fn event_push( events : &mut Vec< Event >, event : Event )
{
  events.push( event );
}

/// Take every event out of `events`, leaving it empty.
#[ must_use ]
pub fn event_drain( events : &mut Vec< Event > ) -> Vec< Event >
{
  core::mem::take( events )
}

/// How many events `events` currently holds.
#[ must_use ]
pub fn event_len( events : &[ Event ] ) -> usize
{
  events.len()
}

/// Discard every event in `events`; unlike [`event_drain`], nothing is
/// returned.
pub fn event_clear( events : &mut Vec< Event > )
{
  events.clear();
}
