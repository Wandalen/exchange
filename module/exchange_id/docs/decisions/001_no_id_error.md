# ADR-001: No `IdError` — Every Conversion Is Infallible

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design's exposed-item list names `IdError { Zero }`
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:535-540`)
alongside the three id types and their `_from_raw`/`_raw` conversions — a
rejected raw value of zero. No real caller anywhere in the family has ever
needed id zero to be invalid; every field on `InstrumentId`/`OrderId`/
`AccountId` stays `pub`, so a fallible `*_from_raw` would sit alongside a
still-available infallible tuple construction (`OrderId( 0 )`) — a check
enforced on one path and silently bypassed on the other.

## Decision

`instrument_from_raw`/`order_from_raw`/`account_from_raw` all return the id
type directly, never `Result`. No `IdError` type is declared.

## Alternatives Considered

### Option 1: Add `IdError::Zero` and make the three `*_from_raw` functions fallible

Rejected: enforcing the check only through `*_from_raw` while every field
stays public and directly constructible (`OrderId( 0 )`) would be a check
with a hole already built into it — the real safety would depend on every
caller choosing the checked path, not on the type system.

### Option 2: Add `IdError::Zero` and make every field private, reachable only through the checked constructors

Rejected: this is a bigger, unrequested API change (removing the public
tuple field every current caller already uses — `exchange_book`,
`exchange_order`, `exchange_level`, and others all construct and read these
ids by direct field/tuple access today) for a validation rule nothing has
ever actually needed.

## Consequences

**Positive:** `instrument_from_raw`/`order_from_raw`/`account_from_raw`'s
signatures say exactly what they do — they cannot fail, and nothing calls
`.unwrap()` to pretend otherwise.

**Negative:** a zero-valued id is constructible and indistinguishable from
any other value. If a real caller ever needs to treat zero as a sentinel or
invalid value, that becomes a breaking signature change then, not now.

## Related

- [`../../../../docs/exposed_item/001_exchange_id_items.md`](../../../../docs/exposed_item/001_exchange_id_items.md) — the source design's own exposed-item entry, naming `IdError { Zero }`
