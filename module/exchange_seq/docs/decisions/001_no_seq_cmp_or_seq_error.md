# ADR-001: No `seq_cmp`, No `SeqError`

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design's exposed-item list names `seq_cmp` and
`SeqError { Exhausted }`
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:567-570`)
alongside `Seq(u64)`/`seq_zero`/`seq_next`. `Sequence` already derives
`PartialOrd`/`Ord`, so a free `seq_cmp` function would wrap `.cmp()` under
a second name — the same "two sources of truth for one fact" pattern
`exchange_types` warns against for `Trade` versus `Fill`. At one increment
per emitted event, exhausting a `u64` counter takes longer than any run of
this exchange will ever last — `SeqError::Exhausted` would model a failure
mode nothing can reach.

## Decision

No `seq_cmp` function — callers compare two `Sequence` values with the
derived `Ord` directly. No `SeqError` type — `seq_next` is infallible.

## Alternatives Considered

### Option 1: Add `seq_cmp` wrapping `Ord::cmp`

Rejected: a second name for the same comparison is exactly the duplicated
fact this family's own conventions warn against elsewhere — see
`exchange_types`' own reasoning for not having a separate `OrderFilled`
event alongside `Trade`.

### Option 2: Add `SeqError::Exhausted` and make `seq_next` fallible

Rejected: there is no test that could ever legitimately exercise this
path — a `u64` counter incrementing once per emitted event would need
more events than any real or synthetic run will ever produce. Modelling an
unreachable failure mode adds a `Result` every caller has to handle for a
case that can never fire.

## Consequences

**Positive:** `seq_next`'s signature says exactly what it does — it cannot
fail — and callers use the derived `Ord` the same way they would for any
other comparable type, with no second comparison API to learn.

**Negative:** none identified — both declined items have no real caller
anywhere in the family today.

## Related

- [`../../../../docs/exposed_item/007_exchange_seq_items.md`](../../../../docs/exposed_item/007_exchange_seq_items.md) — the source design's own exposed-item entry, naming `seq_cmp` and `SeqError`
