# Invariant: Available and Reserved Balance Separation

### Scope

- **Purpose**: Make over-commitment structurally impossible rather than merely checked — you cannot sell what you have already promised, because the units are gone from the spendable side before the second order is ever validated.
- **Responsibility**: State the partition, the four legal edges between its two sides, the conservation clause over their sum, and what a violated partition costs an audit that only sees the sum.
- **In Scope**: The two-sided shape of one account's holding of one asset, and which operations may move quantity across the boundary.
- **Out of Scope**: That a resting order is *fully* covered, which is the coverage property (→ [Escrow Covers Resting Orders](001_escrow_covers_resting_orders.md)); the numeric type, its range, and its rounding (→ [`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_minor/readme.md)); the concurrent-writer hazard on either side (→ [Balance Lost Update](../pitfall/001_balance_lost_update.md)).

### Invariant Statement

An account's holding of an asset is a **pair**, `( available, reserved )`.
Placing an order moves quantity `available → reserved`; only a fill or a
cancel moves quantity out of `reserved`. **`available + reserved` is
invariant under every engine operation** — the sum changes only at an
explicit deposit or withdrawal, which are not on the matching path.

Three clauses:

- **Partition, not annotation.** `reserved` is a *stored* quantity, not a
  view derived by summing the account's resting orders. The book is checked
  against the stored figure; the figure is never recomputed from the book. A
  derived reserve is a second source of truth for one fact, and the two
  disagree exactly when a bug has already occurred — the moment the check is
  least able to notice.
- **Exactly four edges, each carried by an event.** `available → reserved` on
  acceptance; `reserved →` counterparty's `available` on fill;
  `reserved → available` on cancel; and the deposit/withdraw edges that enter
  or leave the pair from outside. There is no fifth edge. In particular
  there is no path that debits `available` while an order rests, and no path
  from `reserved` back to `available` that is not a cancel.
- **Conservation of the pair.** Every engine operation is either a
  rearrangement inside one account's pair or a transfer between two accounts'
  pairs. Summed over all accounts, per asset, `available + reserved` is
  constant across the entire matching path.

**This is the mechanism, not a check.** A check-then-place design reads
`available`, finds it sufficient, and lets the order rest; a second order
reads the same `available` and finds it sufficient too. That is a
time-of-check-to-time-of-use window, and it is open for the whole interval
between the check and whatever writes the balance down. Moving the units at
acceptance closes it by construction: the second order's validation reads an
`available` that has already lost them. The source material reaches the same
conclusion and states the reason plainly — reservations exist so that
"distributed transactions … never ask questions" (→ external design corpus
`initial_design_message/643_yes_uh_right_now_do.md`, plain-text citation,
un-integrated tier), with a worked two-field trace: `Balance = 1500,
Locked = 0` before a 1000-unit commitment, `Balance = 500, Locked = 1000`
after.

**Provenance, stated because it is not uniform.** The two-sided balance is
originated here. The integrated design corpus carries a single-field holding
(→ external design corpus `ecs_component/023_wallet.md`, plain-text
citation) and a separate escrow ledger whose own fields are recorded as "not
fixed by any source" (→ `ecs_component/155_escrow_ledger.md`, plain-text
citation); the split above appears only in the un-integrated message tier. A
reader who assumes the corpus settled this will not look for what it did not
settle.

### Enforcement Mechanism

**Reservation is part of acceptance, release is part of the event.** The
`available → reserved` move happens in
[Price-Time Priority Matching](../algorithm/001_price_time_priority_matching.md)
step 2, before the order is visible to matching; the reverse moves are
carried by the fill and cancel events themselves rather than by a later
settlement pass
(→ [Escrow Covers Resting Orders](001_escrow_covers_resting_orders.md)). This
instance adds nothing to *when* — it adds *what shape the balance has* so
that "reserve" and "release" name field moves rather than bookkeeping.

**Every move is one checked operation on an exact type**
(→ [`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_add/readme.md),
[Conserved Value Type Family](https://github.com/Wandalen/exact/blob/master/module/exact_kind/readme.md)).
`available` cannot be driven negative by a rounding artefact, because there
is no rounding: the quantity subtracted from one side is the identical value
added to the other, never a recomputation of it.

**Both sides of a move happen at one arrival position**
(→ [Arrival Sequence Is Total and Replayable](002_arrival_sequence_total_and_replayable.md)),
from the single-consumer merge path. There is no observable instant in which
quantity has left `available` and not yet arrived in `reserved`
(→ [Balance Lost Update](../pitfall/001_balance_lost_update.md), which is what
happens when a second writer is admitted).

**The bridge to coverage, which is the checkable assertion.** For every
account and asset:

```
reserved  ==  Σ ( maximum remaining obligation of that account's resting
                  orders in that asset )
```

This is the one statement that ties this partition to
[Escrow Covers Resting Orders](001_escrow_covers_resting_orders.md), and it
is the assertion a property test evaluates at every event boundary. Coverage
alone says each order is backed; this says the backing is not double-counted.

**Decided here; open elsewhere.** Three questions are genuinely open and are
named rather than answered:

- **Whether the fee is reserved alongside the notional.** Fees are assessed
  at trade generation, against traded notional
  (→ [Fee Assessment Point](../algorithm/004_fee_assessment_point.md)), so a
  fill can leave an account owing a fee it has no `available` balance to pay.
  Reserving the fee at acceptance fixes that and couples reservation sizing
  to fee policy; not reserving it leaves a settlement path that can fail
  after the trade is already printed. Neither is chosen.
- **Whether `reserved` is one aggregate per `( account, asset )` or one
  figure per order.** Aggregate is cheaper and is all the conservation clause
  needs; per-order makes a cancel's release amount a lookup rather than a
  recomputation, and makes an audit discrepancy attributable to an order
  rather than to an account. The corpus fixes neither.
- **What a market buy reserves.**
  [Price-Time Priority Matching](../algorithm/001_price_time_priority_matching.md)
  step 2 reserves "the order's maximum settlement obligation", and a market
  buy has no limit price, so against a thin or empty book that maximum is
  unbounded. A worst-case notional, a price band, or rejecting a market buy
  that names no cap are the three candidates; none is selected, and the
  choice is not cosmetic — it decides whether a market buy is representable
  at all.

### Violation Consequences

**The same units back two orders.** Both rest, both individually pass a
coverage check against a balance that only covers one. The first to fill
settles normally; the second fills against nothing. The engine then either
mints the shortfall — creating value from nothing, the exact failure
[Escrow Covers Resting Orders](001_escrow_covers_resting_orders.md) exists to
make unrepresentable — or it fails settlement on an order the book displayed
as executable, which breaks the promise that finding a match is the same as
being able to execute it. The fault is at the second *placement*; the symptom
is at the second *fill*, arbitrarily later, against a counterparty who did
nothing wrong.

**Over-release mints, under-release deletes — and the sum clause catches
both.** Releasing more than was reserved credits `available` with units
nobody debited; releasing less strands units in `reserved` with no order
justifying them. Both change `available + reserved`, so both are within the
conservation audit's reach
(→ [Conservation Audit](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md))
— in aggregate, with no attribution, and only whenever the audit next runs.

**A violated *partition* is invisible to that audit, which is the sharper
failure.** A bug that moves quantity `available → reserved` with no order
behind it, or leaves quantity in `reserved` after its order reached a
terminal state, changes neither the sum nor any per-order coverage check. The
audit balances. The account owner simply cannot spend money the system agrees
they own, and there is no error anywhere — the funds are neither usable nor
settled, which is economically a deletion recorded as a holding. This is why
the bridge assertion above is stated as an assertion: the sum is not enough
to detect it, and nothing else will.

**And an incorrect partition survives replay.** Because both sides are
reconstructed from the same event stream
(→ [Trade Event Stream](../protocol/001_trade_event_stream.md)), a replay
reproduces the wrong split exactly as faithfully as it would reproduce the
right one. Recovery is not a repair mechanism for this class.

### Algorithms

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md) | Detects a violated sum; explicitly cannot detect a violated partition, which is why the bridge assertion exists |
| [https://github.com/Wandalen/exact/blob/master/module/exact_dust/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_dust/readme.md) | Why a partial fill's move across the partition is exact rather than two independently rounded figures |
| [../algorithm/001_price_time_priority_matching.md](../algorithm/001_price_time_priority_matching.md) | Step 2 performs the `available → reserved` move; steps 4–5 perform the two reverse moves |
| [../algorithm/003_cancel_and_amend_semantics.md](../algorithm/003_cancel_and_amend_semantics.md) | An amend adjusts the partition in one event — a release-then-reserve decomposition opens the very window this invariant closes |
| [../algorithm/004_fee_assessment_point.md](../algorithm/004_fee_assessment_point.md) | The fee obligation whose reservation status is this instance's first open question |

### Formats

| File | Relationship |
|------|--------------|
| *(no successor — see note)* | Why this partition is only reconstructible after a restart if every reserve/release move was logged as an ordinary entry here; the old `exact_arithmetic` crate's transaction-log-encoding design this once cited was never carried into the real 16-crate [exact](https://github.com/Wandalen/exact) family |

### Invariants

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_add/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_add/readme.md) | Why a cross-partition move cannot drift or drive `available` negative |
| [001_escrow_covers_resting_orders.md](001_escrow_covers_resting_orders.md) | The coverage property this partition is the storage shape for; the bridge assertion is what joins them |
| [002_arrival_sequence_total_and_replayable.md](002_arrival_sequence_total_and_replayable.md) | Both sides of every move happen at one arrival position, which is what makes the pair's conservation replayable |

### Non-Functional Requirements

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_minor/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_minor/readme.md) | The ceiling binding both sides of this partition's sum, not each side separately |

### Pitfalls

| File | Relationship |
|------|--------------|
| [../pitfall/001_balance_lost_update.md](../pitfall/001_balance_lost_update.md) | The concurrent-writer failure that corrupts either side; the split doubles the number of mutable fields per account per asset |

### Protocols

| File | Relationship |
|------|--------------|
| [../protocol/001_trade_event_stream.md](../protocol/001_trade_event_stream.md) | Carries the reserved and released amounts, so a replay reconstructs the partition rather than recomputing it |

### State Machines

| File | Relationship |
|------|--------------|
| [../state_machine/001_order_lifecycle.md](../state_machine/001_order_lifecycle.md) | BI2 and BI5 — reserved reaches zero exactly once, and an order holds escrow if and only if it is on the book |

### Types

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_kind/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_kind/readme.md) | Two balances of one kind per account — moving quantity between them must be same-scale, same-kind, and checked, which this family's algebra guarantees |

### Sources

| File | Relationship |
|------|--------------|
| `ecs_component/023_wallet.md` (design corpus) | The single-field holding this two-sided pair diverges from, recorded so the divergence is not read as a transcription error |
| `initial_design_message/643_yes_uh_right_now_do.md` (design corpus) | The two-field model and its worked trace, and the time-of-check rationale for reserving rather than asking |
| `../../../exchange_escrow/src/lib.rs` | `Holding< T >` — the stored `( available, reserved )` pair and its four private edges |

### Tests

| File | Relationship |
|------|--------------|
| `tests/balance_partition_test.rs` (to create) | Under a randomized place/cancel/fill stream: `available + reserved` per account is constant except across deposits and withdrawals; the bridge assertion holds at every event boundary; two orders totalling more than `available` cannot both be accepted, in either submission order |
