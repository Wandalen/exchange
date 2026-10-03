# ADR-001: No `exchange_id` Dependency

**Date**: 2026-10-02
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design's crate list names `exchange_id` as `exchange_stats`'s one
dependency (`../../../../../../codename_space_sandbox/intake/core_exchange.txt:476-481`).
`BookStats`'s three fields (`rests`, `fills`, `rejects`) are plain running
totals, not keyed by `OrderId`/`AccountId`/`InstrumentId` or any other type
`exchange_id` declares, and none of the four exposed functions
(`stats_zero`/`stats_fill_add`/`stats_reject_add`/`stats_snapshot`) take an id
parameter either, per the source's own exposed-item list
(`core_exchange.txt:647-650`).

## Decision

`exchange_stats` takes the dependency only if a real use for it surfaces —
not now. It stays a zero-dependency root, matching `exchange_tif`,
`exchange_cap`, and `exchange_seq`'s own shape.

## Alternatives Considered

### Option 1: Take the dependency anyway, to match the source design exactly

Rejected: an imported-but-unused dependency is dead weight a reader has to
investigate to rule out, and this workspace's lints treat unused imports as a
warning promoted to an error under `-D warnings` — the dependency could not
even be added without inventing a use for it.

## Consequences

**Positive:** `exchange_stats` stays trivially auditable — one struct, four
pure functions, nothing to trust beyond arithmetic on `u64`.

**Negative:** none identified — no caller today needs a per-id breakdown of
these counts; a future instrument-scoped or account-scoped stats requirement
would need this re-opened, not silently worked around.

## Related

- [`crate/021_exchange_stats.md`](../../../../docs/crate/021_exchange_stats.md) — the source design's own crate entry, naming the dropped dependency
