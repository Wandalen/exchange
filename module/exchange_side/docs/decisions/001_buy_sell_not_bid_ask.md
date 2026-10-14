# ADR-001: Variants Are `Buy`/`Sell`, Not `Bid`/`Ask`

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design names `Side { Bid, Ask }`
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:542`).
The real type has always been `Side { Buy, Sell }` — what the rest of the
family reasons about is the economic action, not the order-book-display
term for it: `exchange_types`' own `obligation` logic branches on "a buy
owes cash, a sell owes the asset," which reads directly off `Buy`/`Sell`
and would need a mental translation off `Bid`/`Ask`.

## Decision

Keep `Side { Buy, Sell }`. `side_opposite`/`side_is_bid`/`side_is_ask` give
the source's own vocabulary a home for callers that want it, without moving
it onto the type itself.

## Alternatives Considered

### Option 1: Rename the variants to `Bid`/`Ask`

Rejected: this would touch every one of the family's crates that match on
`Side` plus workstream 010's consumer, for a label change with no
behavioural benefit — the economic-action framing (`Buy`/`Sell`) is more
directly useful at every real call site than the order-book-display framing
(`Bid`/`Ask`).

## Consequences

**Positive:** matches throughout the family read as the economic action
(`Side::Buy`/`Side::Sell`), and `side_is_bid`/`side_is_ask` still give a
caller that thinks in bid/ask terms the exact predicate it wants.

**Negative:** a reader coming from the source design's own `Bid`/`Ask`
vocabulary has to make the translation once; this ADR and the module doc
comment in [`../../src/lib.rs`](../../src/lib.rs) are both written to make
that translation immediate.

## Related

- [`../../../../docs/exposed_item/002_exchange_side_items.md`](../../../../docs/exposed_item/002_exchange_side_items.md) — the source design's own exposed-item entry, naming `Bid`/`Ask`
- [`../../../../docs/side/001_bid.md`](../../../../docs/side/001_bid.md), [`../../../../docs/side/002_ask.md`](../../../../docs/side/002_ask.md) — the central catalog's own per-value instances, now thinned to point here
