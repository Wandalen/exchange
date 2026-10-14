# ADR-001: No `exchange_book` Dependency

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design's crate list names both `exchange_spec` and `exchange_book`
as `exchange_halt`'s dependencies
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:458-463`).
`halt_set`/`halt_clear`/`halt_is` read and write only
`exchange_spec::InstrumentSpec::halted`; none of the three takes a `Book`
(`core_exchange.txt:631-633`). The halt is enforced by `exchange_core`'s
`step_place`, which refuses a new order on a halted instrument before it
reaches the book.

## Decision

`exchange_halt` depends on `exchange_spec` only.

## Alternatives Considered

### Option 1: Take the dependency anyway, to match the source design exactly

Rejected: no function here has a use for it, so the edge would anchor
nothing.

## Consequences

**Positive:** one flag, three functions, one error type, one dependency.

**Negative:** none so far. When placement-path enforcement landed, it read
`halt_is` from `exchange_core` without a new edge here, as this decision
expected.

## Related

- [`crate/018_exchange_halt.md`](../../../../docs/crate/018_exchange_halt.md) — the source design's own crate entry, naming the dropped dependency
