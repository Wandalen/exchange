# ADR-001: No Constructor, Quantity-Mutation Helpers, or `OrderError` — `Order` Stays an Immutable Plain Struct

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design's exposed-item list names `order_new`, `order_qty_set`,
`order_qty_left`, `order_is_empty`, and a dedicated `OrderError { ZeroQty, BadId }`
alongside `Order` itself
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:561-566`).
The real build declares none of the five.

## Decision

Every field on `Order` stays `pub`; the only way to build one is a plain
struct literal. `Order.quantity` never mutates after construction. No
`OrderError` type exists.

## Alternatives Considered

### Option 1: Build `order_new` as a thin wrapper over the struct literal

Rejected: with every field already `pub` and no invariant a constructor
could enforce that the type system doesn't already guarantee (every field
is a plain value type, not a reference needing validation), `order_new`
would be a second construction path with nothing to validate — a pure
duplicate of the struct literal, not a safer alternative to it.

### Option 2: Add `order_qty_set`/`order_qty_left`/`order_is_empty` as mutators on `Order`

Rejected: these presuppose quantity shrinks on `Order` itself as fills
land. The real design instead keeps `order.quantity` fixed for the order's
whole lifetime — "what was submitted" — and tracks the shrinking remainder
on a separate wrapper, `exchange_book::Resting`/`exchange_level::LevelNode`,
created only once an order actually rests. Adding mutators to `Order` would
create two places quantity could live and drift against each other; only a
resting order has a remainder to track in the first place, so the wrapper
is the right home, not this crate.

### Option 3: Build `OrderError { ZeroQty, BadId }` for construction-time validation

Rejected: a zero-quantity order is already refused one layer up, at
`exchange_core::Exchange::submit`'s boundary, via the existing
`RejectReason::ZeroQuantity` (`module/exchange_core/src/lib.rs:235-238`) —
the same reject-reason vocabulary every other submission failure already
uses. A second, construction-time error type would duplicate that check
under a different name rather than add a new one; `BadId` has no real
construction-time check to attach to, since `OrderId` is a plain newtype
with no invalid values to reject.

## Consequences

**Positive:** one unambiguous way to build an `Order`, with no fallible
constructor to keep in sync with the struct's field list as it evolves;
quantity has exactly one owner at every point in an order's life
(`Order.quantity` before it rests, `Resting.remaining`/`LevelNode.remaining`
after).

**Negative:** nothing at the type level stops a caller from constructing an
`Order` with a zero `quantity` directly — the check only fires once that
order reaches `exchange_core::Exchange::submit`, not at the point of
construction. Accepted: no code path builds an `Order` without intending to
submit it.

## Related

- [`../../../../docs/exposed_item/006_exchange_order_items.md`](../../../../docs/exposed_item/006_exchange_order_items.md) — the source design's own exposed-item entry, naming all five
- [`../../../exchange_level/docs/decisions/001_no_level_error.md`](../../../exchange_level/docs/decisions/001_no_level_error.md) — the sibling decision on `LevelError`, same "handled one layer up" shape
