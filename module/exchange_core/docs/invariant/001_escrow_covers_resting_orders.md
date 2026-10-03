# Invariant: Escrow Covers Resting Orders

### Scope

- **Purpose**: Guarantee that no order the book displays can fail settlement, so a match is always executable the instant it is found.
- **Responsibility**: State the coverage property, the reservation/release discipline enforcing it, and what an unbacked or leaked reservation costs.
- **In Scope**: The lifetime of a reservation — from order acceptance through rest, fill, and cancel.
- **Out of Scope**: The exactness of reservation arithmetic (→ [`exact_arith`'s checked operations](https://github.com/Wandalen/exact/blob/master/module/exact_add/readme.md)); matching order and TIF disposition (→ [Price-Time Priority Matching](../algorithm/001_price_time_priority_matching.md)).

### Invariant Statement

At every observable point between events, **every resting order is backed by
an escrow reservation covering its maximum remaining settlement
obligation** — and a fill or cancel **releases exactly the matched or
cancelled amount in the same event**, never in a later one.

Two directions, both binding:

- **Never under-covered.** No order is visible in the book for any interval,
  however short, during which its full remaining obligation is not reserved.
- **Never over-held.** No reservation outlives the order state that
  justifies it; release is part of the fill/cancel event itself, not an
  asynchronous settlement step that can lag or be lost.

### Enforcement Mechanism

**Reservation precedes book insertion.** Accepting an order reserves its
maximum obligation first; only a successfully reserved order becomes visible
to matching. An order that cannot be fully reserved is rejected whole — there
is no partially-backed rest state. This is the same all-or-nothing shape the
design corpus states as an explicit ACID requirement on the engine —
processing transactions under strict consistency, where a transfer "must go
through completely or not at all" (→ external design corpus
`system/032_order_matching_engine.md`, plain-text citation) — applied here
to reservation rather than to settlement.

**Release is event-atomic.** The fill event that reduces an order's
remainder carries the corresponding release; the cancel event carries the
release of everything remaining. Because recovery is event-sourced
(→ [Exchange Core v0.1](../feature/001_exchange_core_v0_1.md)), replaying
events reproduces reservations exactly — there is no side channel where a
release can exist without its event.

**Arithmetic cannot drift.** Reservations, fills, and releases are
[`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md)
values with checked operations, and partial fills split
conservation-exact — so "maximum remaining obligation" is always a computable
exact quantity, never an estimate with rounding slack.

### Violation Consequences

- **An unbacked fill mints value.** The trade settles against funds that
  were never locked, creating wealth from nothing — the zero-sum law the
  whole economy rests on is broken, and the conservation audit
  (→ [Conservation Audit](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md))
  catches it only after the fact, as an unattributable imbalance.
- **A leaked reservation locks wealth out of existence.** Funds neither
  usable by their owner nor settled to a counterparty — economically a
  deletion, which the same audit reports as the mirror-image imbalance.
- Both failures are silent at the point of fault and loud only at audit
  time, far from the event that caused them — which is why the property is
  enforced by construction (reservation-before-rest, release-in-event) rather
  than checked by reconciliation.

### Algorithms

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md) | The after-the-fact detector for both violation directions |
| [../algorithm/001_price_time_priority_matching.md](../algorithm/001_price_time_priority_matching.md) | Steps 2 and 5 of the matching flow are where this invariant's reserve and release actually happen |
| [../algorithm/003_cancel_and_amend_semantics.md](../algorithm/003_cancel_and_amend_semantics.md) | An amend adjusts the reservation inside its own event, and a rejected increase must leave the order untouched |
| [../algorithm/002_self_match_prevention.md](../algorithm/002_self_match_prevention.md) | The executability promise this policy protects — skip-and-continue would break it by leaving an untradeable order displayed |

### Features

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md) | Reservation, fill, and release arithmetic is this feature's exact type substrate — no drift possible in the coverage amount |
| [../feature/001_exchange_core_v0_1.md](../feature/001_exchange_core_v0_1.md) | Exit criterion 3 is this invariant under a randomized stream |

### Invariants

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_add/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_add/readme.md) | Why reservation arithmetic cannot silently drift the coverage amount |
| [003_available_and_reserved_balance_separation.md](003_available_and_reserved_balance_separation.md) | The two-sided holding this coverage property is stated over; its bridge assertion is what stops one reservation backing two orders |

### Pitfalls

| File | Relationship |
|------|--------------|
| [../pitfall/001_balance_lost_update.md](../pitfall/001_balance_lost_update.md) | Why coverage still passes over a corrupted ledger — this property is stated over orders still on the book |

### Protocols

| File | Relationship |
|------|--------------|
| [../protocol/001_trade_event_stream.md](../protocol/001_trade_event_stream.md) | Carries the reserved and released amounts inside the same events, which is what makes release event-atomic rather than a later pass |

### State Machines

| File | Relationship |
|------|--------------|
| [../state_machine/001_order_lifecycle.md](../state_machine/001_order_lifecycle.md) | BI5 states this coverage as a state property; BI2 states the release-exactly-once half |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs` | `Exchange::submit` reserves before the book sees the order, and rejects whole |
| `../../../exchange_escrow/src/lib.rs` | The reservation ledger this invariant constrains, and its reserve-or-reject-whole path |
| `system/032_order_matching_engine.md` (design corpus) | States the ACID all-or-nothing requirement this invariant's reserve-or-reject-whole rule applies to reservation |

### Tests

| File | Relationship |
|------|--------------|
| `tests/escrow_test.rs` (to create) | Property test: under a randomized order/cancel/fill stream, coverage holds at every event boundary and total reserved equals the sum of resting maximum obligations |
