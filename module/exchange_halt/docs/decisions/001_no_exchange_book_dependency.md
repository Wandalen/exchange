# ADR-001: No `exchange_book` Dependency

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design's crate list names both `exchange_spec` and `exchange_book`
as `exchange_halt`'s dependencies
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:458-463`).
`halt_set`/`halt_clear`/`halt_is` operate only on
`exchange_spec::InstrumentSpec`'s existing `halted : bool` field — none of
the three touch a `Book` or anything book-shaped, and nothing in the real
match loop (`exchange_match::cross`) or the real resting path checks
`halted` yet. The wiring that would give a `Book` dependency a real purpose —
refusing a match or a rest against a halted instrument — belongs to whichever
stage reworks `exchange_match`/`exchange_rest`, the same gap this family's
own `exchange_cap::BookCaps` already documents ("not yet consulted by any
real crate").

## Decision

`exchange_halt` depends on `exchange_spec` only. The `exchange_book`
dependency is not taken until a real caller inside this crate needs it.

## Alternatives Considered

### Option 1: Take the dependency anyway, to match the source design exactly

Rejected: an imported-but-unused dependency has nothing to anchor it — not
one function signature in this crate's own exposed-item list
(`core_exchange.txt:631-633`) takes a `Book` or `&Book` parameter. Adding it
now would be a dependency this crate's own code can't point to a use for.

## Consequences

**Positive:** `exchange_halt` stays a small, two-dependency-free-of-book
crate — one flag, three functions, one error type.

**Negative:** none identified now — the eventual match-loop/rest-path wiring
will need its own access to `InstrumentSpec::halted` (via `halt_is` or
directly), not a new dependency edge from `exchange_halt` itself, so this
decision does not need to be revisited when that wiring lands.

## Related

- [`crate/018_exchange_halt.md`](../../../../docs/crate/018_exchange_halt.md) — the source design's own crate entry, naming the dropped dependency
