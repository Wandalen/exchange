# ADR-001: No `exchange_order` Dependency

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design's crate list names `exchange_order` and `exchange_book` as
`exchange_snap`'s dependencies
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:470-475`).
At the time this crate was scoped, `exchange_order` did not yet exist in the
real build; by the time it was implemented, a concurrent session had built
it. Re-checked against the real, now-existing `exchange_order` crate rather
than assumed stale: its own public surface
(`module/exchange_order/src/lib.rs`) is `Order`/`Obligation`, and nothing in
`exchange_snap`'s design needs either — [`RestRow`] holds a plain
`exchange_id::OrderId`, not a full `Order`, since a snapshot row only needs
to name *which* order rests, not carry its whole record a second time.

## Decision

`exchange_snap` depends on `exchange_book` and `exchange_id` (for `OrderId`
and `InstrumentId`). `exchange_order` is not taken, now or provisionally —
nothing in this crate's own code would reference `Order`/`Obligation`.

## Alternatives Considered

### Option 1: Take the dependency anyway, to match the source design exactly

Rejected: same reasoning as `exchange_stats`'s own
[dropped `exchange_id` dependency](../../../exchange_stats/docs/decisions/001_no_exchange_id_dependency.md)
— an imported-but-unused dependency is dead weight with nothing in this
crate's code to anchor it.

## Consequences

**Positive:** `exchange_snap` depends only on the two crates its own code
actually names.

**Negative:** none identified — if a later revision of `RestRow` needs the
full `Order` (not just its id), this decision is the one to revisit, not a
silent dependency creep.

## Related

- [`crate/020_exchange_snap.md`](../../../../docs/crate/020_exchange_snap.md) — the source design's own crate entry, naming the dropped dependency
- [`002_no_snap_error.md`](002_no_snap_error.md) — the companion decision on the dropped `SnapError` type
