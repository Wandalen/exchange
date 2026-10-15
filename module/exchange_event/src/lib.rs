//! The one owned drain point for the event stream, for workstream 010 to
//! read fills, rejects, and cancel-acks from without writing wallets
//! directly. Closes hard problem 12 (events, not wallets) and feature 26
//! (event drain for 010).
//!
//! # Why this crate exists despite being marked "Folded" centrally
//!
//! `docs/crate/019_exchange_event.md` records `Event`/`EventKind` as already
//! folded into `exchange_types` (since moved on again to `exchange_fill` —
//! see that crate's own module doc), with `exchange_core::events()` serving
//! as the accessor — true, and not re-litigated here. But `events()` returns
//! a borrowed `&[Event]` (`exchange_core/src/lib.rs`), never an owned drain:
//! feature 26 names "the one drain point," and a caller holding only a
//! borrow has no way to take events out and leave the source empty, the way
//! an actual queue drain does. [`event_drain`] is that still-missing piece —
//! genuinely new, not a re-implementation of anything `exchange_fill` or
//! `exchange_core` already provides.
//!
//! # Re-exports, doesn't redefine
//!
//! [`Event`]/[`EventKind`] are re-exported from `exchange_fill` unchanged —
//! this crate adds the functions the proposal's own `exchange_event` names
//! (`core_exchange.txt:636-639`: `event_push`, `event_drain`, `event_len`,
//! `event_clear`), without a second definition of the types they operate on.
//!
//! # No `EventDrain` type, no `EventError { Full }`
//!
//! The proposal also names a dedicated `EventDrain` container and a
//! capacity-bounded `EventError::Full`. Neither is built: the real system's
//! own event storage (`exchange_core::Exchange`'s internal field,
//! `events()`'s own `&[Event]` return) is a plain, unbounded `Vec<Event>`,
//! with no capacity concept anywhere in the real design for a `Full` error
//! to report — the same reasoning `exchange_snap`'s own
//! `docs/decisions/002_no_snap_error.md` gives for declining its own
//! proposed `SnapError::Full`. Every function below operates on a plain
//! `&mut Vec<Event>` directly, matching that real representation, rather
//! than introducing a wrapper type the rest of the system doesn't use.

pub use exchange_fill::{ Event, EventKind };

/// Append one event to `events`.
pub fn event_push( events : &mut Vec< Event >, event : Event )
{
  events.push( event );
}

/// Take every event out of `events`, leaving it empty.
///
/// The owned counterpart to `exchange_core::Exchange::events()`'s borrowed
/// slice: a caller that wants to consume the stream once, rather than read
/// it repeatedly, calls this instead of cloning a borrowed slice by hand.
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

/// Discard every event in `events` without returning them.
///
/// Unlike [`event_drain`], the discarded events are not recoverable — for a
/// caller that wants to reset the stream, not consume it.
pub fn event_clear( events : &mut Vec< Event > )
{
  events.clear();
}
