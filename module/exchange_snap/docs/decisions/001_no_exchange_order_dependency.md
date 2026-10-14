# ADR-001: No `exchange_order` Dependency

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design names `exchange_order` and `exchange_book` as
`exchange_snap`'s dependencies
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:470-475`).
`exchange_order`'s surface is `Order`/`Obligation`, and nothing here needs
either: a [`RestRow`] names its order by `exchange_id::OrderId` rather than
carrying the whole record a second time.

## Decision

`exchange_snap` depends on `exchange_book`, `exchange_id` and
`exchange_side`, not on `exchange_order`.

## Alternatives Considered

### Option 1: Take the dependency anyway, to match the source design exactly

Rejected: same reasoning as `exchange_stats`'s own
[dropped `exchange_id` dependency](../../../exchange_stats/docs/decisions/001_no_exchange_id_dependency.md)
— an unused dependency is dead weight.

## Consequences

**Positive:** `exchange_snap` depends only on the crates its code names.

**Negative:** none identified — if `RestRow` ever needs the full `Order`,
this decision is the one to revisit.

## Related

- [`crate/020_exchange_snap.md`](../../../../docs/crate/020_exchange_snap.md) — the source design's own crate entry, naming the dropped dependency
- [`002_no_snap_error.md`](002_no_snap_error.md) — the companion decision on the dropped `SnapError` type
