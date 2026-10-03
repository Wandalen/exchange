# ADR-002: No `EventError` — Every Function Is Infallible

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design's exposed-item list names `EventError { Full }`
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:636-639`)
alongside `event_push`. A `Full` variant only has meaning against a
bounded-capacity queue — some fixed-size buffer `event_push` could overrun.
No such bounded type exists anywhere in the real build: `exchange_core`'s
own event storage is an unbounded `Vec<Event>`, matching
[`001_no_event_drain_type.md`](001_no_event_drain_type.md)'s own finding.
This is the same reasoning `exchange_snap`'s
[`../../../exchange_snap/docs/decisions/002_no_snap_error.md`](../../../exchange_snap/docs/decisions/002_no_snap_error.md)
gives for declining its own proposed `SnapError::Full`.

## Decision

`event_push`/`event_drain`/`event_len`/`event_clear` all return a plain
value, never `Result`. No `EventError` type is declared.

## Alternatives Considered

### Option 1: Add a fixed-capacity event buffer and a real `Full` case

Rejected for now: same reasoning as `exchange_snap`'s ADR-002 — this is a
family-wide capacity-bounding decision (a chosen capacity, an eviction or
refusal policy), not a one-crate choice, and nothing today needs it. The
real event stream has never once overflowed in practice because nothing
bounds it.

## Consequences

**Positive:** `event_push`'s signature says exactly what it does — it
cannot fail.

**Negative:** if a bounded event stream is adopted later, `event_push`'s
signature changes from infallible to fallible — a breaking change for any
caller by then. Revisit this decision first if that direction is taken.

## Related

- [`../../../../docs/exposed_item/019_exchange_event_items.md`](../../../../docs/exposed_item/019_exchange_event_items.md) — the source design's own exposed-item entry, naming `EventError { Full }`
- [`001_no_event_drain_type.md`](001_no_event_drain_type.md) — the companion decision on the dropped `EventDrain` type
