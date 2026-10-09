# ADR-001: No `Allow`; `CancelResting`/`CancelIncoming` Naming; `CancelBoth` Added

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: wandalen

## Context

The source design names `Stp { Allow, CancelOldest, CancelNewest }` with a
`stp_name` display helper
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:550-552`).
The real `SelfMatchPolicy` has three variants, but not the same three:
`CancelResting`, `CancelIncoming`, `CancelBoth`. Self-match prevention is
unconditional throughout this family — `exchange_match`'s own module
documentation states it is "fixed, never a mode" — so there is no code path
an `Allow` variant could ever select.

## Decision

No `Allow` variant — a self-match is always refused, never permitted to
cross. The two real cancellation variants are named by the order's *role* in
the cross (`CancelResting`/`CancelIncoming`) rather than by arrival time
(`CancelOldest`/`CancelNewest`): in a self-match the resting order is always
the older one and the incoming order is always the newer one, so "cancel
resting" and "cancel oldest" withdraw the same order under two different
descriptions — likewise `CancelIncoming` and `CancelNewest`. A third real
variant, `CancelBoth` (withdraw both remainders), has no counterpart in the
proposal's three at all. `stp_name` is built, matching the proposal exactly.

## Alternatives Considered

### Option 1: Build `Allow` as a real permissive mode

Rejected: this would be new, unrequested permission for an account to trade
with itself — not an extraction of something already true, but a behavior
change contradicting `exchange_match`'s unconditional-refusal design.

### Option 2: Name the two real variants `CancelOldest`/`CancelNewest` to match the proposal literally

Rejected: the chosen names describe the order's role in the cross, which is
what every real call site actually reasons about (is this the order already
resting, or the one just arriving) — the arrival-time framing is accurate
only because of the role framing, never the reverse, so the role names
carry the primary fact and the arrival-time names would carry a derived one.

### Option 3: Omit `CancelBoth`, keep only two variants

Rejected: `CancelBoth` is already real, tested, and distinct
(`tests/exchange_stp_test.rs`) — removing it would be deleting a working
policy to force a closer match to a proposal that never anticipated it,
with no actual benefit.

## Consequences

**Positive:** the enum's own names explain the mechanism (which order's role
is being withdrawn) rather than requiring a reader to already know which
order arrives first in a self-match to decode "oldest"/"newest". `stp_name`
gives a caller that wants the proposal's own display vocabulary a direct
match (`"cancel_resting"`/`"cancel_incoming"`/`"cancel_both"`).

**Negative:** a reader coming from the source design's own `Allow`/
`CancelOldest`/`CancelNewest` vocabulary has to make two translations (the
missing `Allow`, and the renamed pair) at once; this ADR and the module doc
comment in [`../../src/lib.rs`](../../src/lib.rs) are both written to make
that translation immediate.

## Related

- [`../../../../docs/exposed_item/004_exchange_stp_items.md`](../../../../docs/exposed_item/004_exchange_stp_items.md) — the source design's own exposed-item entry, naming `Stp { Allow, CancelOldest, CancelNewest }`
- [`../../../../docs/stp_policy/`](../../../../docs/stp_policy/readme.md) — the central catalog's own per-value instances, now thinned to point here
