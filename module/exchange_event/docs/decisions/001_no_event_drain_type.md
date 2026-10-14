# ADR-001: No `EventDrain` Type — Functions Operate on `Vec<Event>` Directly

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design's exposed-item list names a dedicated `EventDrain` type
alongside `event_push`/`event_drain`/`event_len`/`event_clear`
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:636-639`).
The real event storage is a plain `Vec<Event>` — `exchange_core::Exchange`'s
own field, lent out by `events()` and taken by `event_drain()`.

## Decision

`event_push`/`event_drain`/`event_clear` take `&mut Vec<Event>`, `event_len`
takes `&[Event]`. No `EventDrain` struct is declared.

## Alternatives Considered

### Option 1: Wrap `Vec<Event>` in a new `EventDrain` struct

Rejected: `Exchange` would store the wrapper instead of its `Vec` and gain no
behaviour the `Vec` lacks. A wrapper earns its place once the stream is
bounded, which [`002_no_event_error.md`](002_no_event_error.md) declines.

## Consequences

**Positive:** `Exchange::event_drain` is a direct call on its own field.

**Negative:** nothing stops a caller passing a `Vec<Event>` that is not the
exchange's own stream.

## Related

- [`../../../../docs/exposed_item/019_exchange_event_items.md`](../../../../docs/exposed_item/019_exchange_event_items.md) — the source design's own exposed-item entry, naming `EventDrain`
- [`002_no_event_error.md`](002_no_event_error.md) — the companion decision on the dropped `EventError`
