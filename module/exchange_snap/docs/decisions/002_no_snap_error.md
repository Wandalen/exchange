# ADR-002: No `SnapError` — `snap_take` Is Infallible

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design's exposed-item list names `SnapError { Full }`
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:641-645`)
alongside `RestRow`/`BookSnap`/`snap_take`/`snap_len`. A `Full` variant only
has meaning against a bounded-capacity row collection — some fixed-size
buffer that `snap_take` could overrun. No such bounded type exists anywhere
in the real build today: `exchange_book::Book` itself is two unbounded
`Vec<Resting>`s, and the crate's own purpose line closes hard problem 16
("closed types"), which is an aspiration this crate alone cannot retire —
introducing a fixed-capacity `BookSnap::rows` here, with nothing else in the
family actually bounded yet, would be a new architectural pattern invented
for one crate rather than an adopted family-wide convention.

## Decision

`BookSnap::rows` is a plain `Vec<RestRow>`, and `snap_take` returns `BookSnap`
directly rather than `Result<BookSnap, SnapError>`. No `SnapError` type is
declared.

## Alternatives Considered

### Option 1: Add a fixed-capacity row buffer and a real `Full` case

Rejected for now: this is a legitimate direction (per hard problem 16), but
it is a family-wide capacity-bounding decision, not a one-crate choice — it
would need a chosen capacity, a policy for what happens to the orders that
don't fit (silently dropped? the oldest evicted? the whole snapshot
refused?), and a precedent other "closed types" crates in this family would
need to follow too. None of that is settled today, and inventing it
unprompted for this one crate would be speculative scope beyond what any
concrete caller currently needs.

### Option 2: Keep `SnapError` but give it a different, achievable meaning

Rejected: there is no other failure mode in `snap_take` today — it never
reads anything fallible (`tick` is accepted as-is, `instrument` is accepted
as-is, every row comes from an already-valid `Resting`). Keeping the type
with no real variant would be exactly the padding this family's own
`exact_arith` facade doc warns against: "the first such helper is how a
facade turns into a fifteenth implementation."

## Consequences

**Positive:** `snap_take`'s signature says exactly what it does — it cannot
fail, and nothing calls `.unwrap()` to pretend otherwise.

**Negative:** if a bounded-capacity `BookSnap` is adopted later (Option 1),
`snap_take`'s signature changes from infallible to fallible — a breaking
change for any caller by then, not an additive one. Revisit this decision
first if that direction is taken, rather than bolting capacity on silently.

## Related

- [`exposed_item/020_exchange_snap_items.md`](../../../../docs/exposed_item/020_exchange_snap_items.md) — the source design's own exposed-item entry, naming `SnapError { Full }`
- [`001_no_exchange_order_dependency.md`](001_no_exchange_order_dependency.md) — the companion decision on the dropped dependency
