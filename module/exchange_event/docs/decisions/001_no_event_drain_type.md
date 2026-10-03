# ADR-001: No `EventDrain` Type — Functions Operate on `Vec<Event>` Directly

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design's exposed-item list names a dedicated `EventDrain` type
alongside `event_push`/`event_drain`/`event_len`/`event_clear`
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:636-639`).
The real system's own event storage, though, is a plain `Vec<Event>` —
`exchange_core::Exchange`'s internal field, surfaced read-only via
`events( &self ) -> &[Event]` (`module/exchange_core/src/lib.rs`). No
wrapper queue type exists anywhere in the real build for `EventDrain` to be
an interface over.

## Decision

`event_push`/`event_drain`/`event_len`/`event_clear` all take `&mut Vec<Event>`
(or `&[Event]` for the read-only `event_len`) directly. No `EventDrain`
struct is declared.

## Alternatives Considered

### Option 1: Wrap `Vec<Event>` in a new `EventDrain` struct

Rejected: this crate is explicitly a skeleton, not wired into
`exchange_core` in this pass (see this work item's own plan addendum,
"Skeleton, not full integration"). Introducing `EventDrain` now would mean
either `exchange_core` keeps its own separate `Vec<Event>` field and this
type sits unused, or this pass reaches into `exchange_core`'s internals to
swap its storage — the latter is exactly the facade rework this plan
reserves for a later stage, not this one.

## Consequences

**Positive:** every function here operates on exactly the type
`exchange_core::Exchange::events()` already exposes a borrow of — wiring
this crate in later is a drop-in call, not a representation change.

**Negative:** no encapsulation prevents a caller from pushing/draining a
`Vec<Event>` that isn't actually the exchange's own event stream — the
functions trust the caller to pass the right `Vec`, same as `event_drain`
already did before this ADR.

## Related

- [`../../../../docs/exposed_item/019_exchange_event_items.md`](../../../../docs/exposed_item/019_exchange_event_items.md) — the source design's own exposed-item entry, naming `EventDrain`
- [`002_no_event_error.md`](002_no_event_error.md) — the companion decision on the dropped `EventError`
