# Algorithm: Fee Assessment Point

### Scope

- **Purpose**: Fix the single point in the matching path at which a fee is assessed, before an implementation puts it somewhere that turns cancelling into a revenue event or charges a partial-fill sequence twice.
- **Responsibility**: State the assessment point and its argument, require the maker/taker classification without fixing a rate, and route the split arithmetic to the crate that owns exactness.
- **In Scope**: Where a fee is computed, what it is computed against, which side is which, and where the assessed amount is recorded.
- **Out of Scope**: The rate and its schedule, which are policy inputs and not this crate's to choose; the arithmetic of an uneven split (→ [Conservation-Exact Split](https://github.com/Wandalen/exact/blob/master/module/exact_dust/readme.md)); the executed price a notional is computed from, which stays open (→ [Price-Time Priority Matching](001_price_time_priority_matching.md)); how the fee amount is written down (no successor doc — the old `exact_arithmetic` crate's transaction-log-encoding design was never carried into the real 16-crate [exact](https://github.com/Wandalen/exact) family).

### Abstract

A fee is assessed at **exactly one point**: when a trade is generated, against
that trade's own notional. Not at order placement, not at cancel, not at
amend, and not a second time when a partial-fill sequence completes.

Everything else about fees is left open on purpose — no rate, no schedule, no
tier. What is decided here is *where* and *against what*, because those two
are the parts that cannot be changed later without changing the meaning of
every archived event, and because both alternatives to them create incentives
that survive long after the code that introduced them.

The source material offers exactly one concrete rate — a 5% figure at a
single venue (→ external design corpus `faction/007_genesis_state.md`,
plain-text citation) — and it already conflicts with the same corpus's
generic fee mechanism, which records its own rate as unspecified
(→ `ecs_system/054_fee_collection_system.md`, plain-text citation). One
conflicting figure is not a rate to freeze into a general-purpose engine.
What the corpus does fix, and this instance adopts, is that assessment
happens on executed deals rather than on submitted ones.

### Algorithm

1. **Assess at trade generation.** Inside
   [Price-Time Priority Matching](001_price_time_priority_matching.md)
   step 3, at the moment a fill is produced, the fee for that fill is computed
   against that fill's own notional — executed price × executed quantity — and
   nowhere else in the pipeline. Steps 1, 2, 5, and every operation in
   [Cancel and Amend Semantics](003_cancel_and_amend_semantics.md) assess
   nothing.

2. **Classify both sides.** Every fill has exactly one **resting** side and
   one **aggressing** side; the resting order is the maker, the incoming order
   is the taker. This is a *classification*, not a price: the engine must be
   able to state which is which, and does not decide what either is charged.
   The classification is total — the match loop cannot produce a fill without
   exactly one of each — and it falls out of the loop's own structure rather
   than from a heuristic, which is why it costs nothing to record and is not
   reconstructable later if it is not.

3. **Take the rate as a policy input, per book.** The engine applies whatever
   rate it is configured with and holds no opinion about its value. **Open:**
   the shape of that input — a flat rate, a maker/taker pair, a
   volume-tiered schedule, or a per-account override. All four fit the
   assessment point above without changing it, which is what makes leaving the
   shape open cheap.

4. **Hand the split arithmetic to `exact_arith`.** A fee that does not
   divide evenly must not create or destroy units; the split is performed by
   [Conservation-Exact Split](https://github.com/Wandalen/exact/blob/master/module/exact_dust/readme.md)
   over
   [conserved value types](https://github.com/Wandalen/exact/blob/master/module/exact_kind/readme.md),
   inside the representable range and scale that crate budgets
   (→ [Representable Range and Scale Budget](https://github.com/Wandalen/exact/blob/master/module/exact_minor/readme.md)).
   This crate supplies the notional and the classification and consumes an
   exact result; it states no scale and performs no rounding of its own.

   The failure this avoids is specific and easy to reach: computing
   `notional × rate` and rounding each side independently mints or destroys up
   to one unit per fill. Per trade it is invisible. In aggregate it is
   unbounded, it is signed consistently by whichever way the rounding leans,
   and it presents to the conservation audit as an unattributable drift
   (→ [Conservation Audit](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md)).

5. **Record the assessed amount as a field of the `Trade` event**, per side
   (→ [Trade Event Stream](../protocol/001_trade_event_stream.md)) — never as
   a separately emitted fee event. A separate event would be a second record
   of one transfer, and a consumer that applied both would double-debit, in
   exactly the replay path the audit depends on.

6. **A fee is a transfer, not a burn.** The assessed units leave the trading
   accounts and arrive at a named fee account, like any other transfer, so the
   conservation audit sees both ends. A fee that simply vanished would present
   as destruction and would have to be special-cased in the one check that
   must have no special cases. **Open:** whether the fee account is one per
   book or one per venue — a bookkeeping question with no effect on the
   assessment point.

**Why not at placement.** Assessing when an order is accepted makes *placing*
a revenue event, with three concrete consequences. Cancelling becomes
revenue-positive for the venue, so the operator is paid for order flow that
never trades and the incentive runs toward churn — a fee model that rewards a
market for not working. The fee must then be reserved at acceptance, which
enlarges every reservation by an amount depending on the rate and couples
escrow sizing to fee policy; that closes
[Available and Reserved Balance Separation](../invariant/003_available_and_reserved_balance_separation.md)'s
open question about reserving fees in the worst available direction, by making
the answer mandatory rather than chosen. And a rejected order's fee must be
refunded, adding a refund path whose only purpose is undoing an assessment
that should not have occurred.

**Why not at cancel.** The same defect, sharper: it prices the *withdrawal* of
liquidity, so a participant is charged for correcting a stale quote. It also
turns self-match prevention into a revenue lever — every cancellation issued
by [Self-Match Prevention](002_self_match_prevention.md) would earn the venue
a fee for a trade it forbade, which is a policy the engine would then be paid
to trigger.

**Why the per-fill rule needs no partial-fill special case.** An order filling
in five parts is charged five fees, on five notionals summing to the whole
executed notional. Because the fee is a function of the fill, and every
executed unit belongs to exactly one fill
(→ [Order Lifecycle](../state_machine/001_order_lifecycle.md) BI4:
`remaining == submitted − Σ fills`, exactly), the total fee is a function of
the total executed quantity **regardless of how it was split**. Every
alternative assessment point has to write a partial-fill rule; this one does
not, and the absence of that rule is the absence of somewhere for a
double-charge to hide.

That invariance also closes an arbitrage. If the total fee varied with the
number of fills, an account could reduce it by aiming at a thicker or thinner
book — turning execution quality into a fee optimisation and giving order
routing a reason to prefer worse fills.

**Price improvement — named, not adopted.** The source material's worked fill
has a 10 000 000-unit bid at 1.1200 filling 2 600 000 against resting asks at
1.1200, 1.1198, and 1.1197, and rules that the resulting price difference
accrues to the venue with commission charged on top (→ external design corpus
`conversation/anatolii_shlyakhto/001_capstone_recruitment_thread.md`,
plain-text citation). That is a **second revenue component**, distinct from
the fee, and this instance does not adopt it — because it is not a fee
decision at all. It is a consequence of the executed-price rule, which
[Price-Time Priority Matching](001_price_time_priority_matching.md) already
names open: if each fill executes at the resting order's price, the aggressor
keeps the improvement and the second component does not exist; if it executes
at the aggressor's limit, the venue captures it. Whichever way that rule
settles, this instance fixes only that **the fee is one assessment against
the executed notional**, whatever executed price that rule produces.

### Algorithms

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md) | Sees both ends of the fee transfer, which is why a fee is never a burn |
| [https://github.com/Wandalen/exact/blob/master/module/exact_dust/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_dust/readme.md) | Performs the uneven split; owns the guarantee this crate consumes rather than restates |
| [001_price_time_priority_matching.md](001_price_time_priority_matching.md) | Hosts the single assessment point at step 3, and owns the still-open executed-price rule the notional is computed from |
| [002_self_match_prevention.md](002_self_match_prevention.md) | Why a cancel-time fee would pay the venue for its own prohibition, and why an executed self-trade is a pure fee leak |
| [003_cancel_and_amend_semantics.md](003_cancel_and_amend_semantics.md) | The operations that deliberately assess nothing |

### Features

| File | Relationship |
|------|--------------|
| [../feature/001_exchange_core_v0_1.md](../feature/001_exchange_core_v0_1.md) | Lists fee computation in scope with no semantics committed; this instance commits the point and the classification, and no rate |

### Invariants

| File | Relationship |
|------|--------------|
| [../invariant/003_available_and_reserved_balance_separation.md](../invariant/003_available_and_reserved_balance_separation.md) | Its open question — whether the fee is reserved alongside the notional — which assessing at placement would close in the worst direction |

### Non-Functional Requirements

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_minor/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_minor/readme.md) | The scale a rate-multiplied notional must land inside, which is why this crate states none of its own |

### Protocols

| File | Relationship |
|------|--------------|
| [../protocol/001_trade_event_stream.md](../protocol/001_trade_event_stream.md) | Carries the per-side assessed amount as a `Trade` field; why no separate fee event exists |

### State Machines

| File | Relationship |
|------|--------------|
| [../state_machine/001_order_lifecycle.md](../state_machine/001_order_lifecycle.md) | BI4 — every executed unit belongs to exactly one fill, which is what makes the total fee invariant under splitting |

### Types

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_kind/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_kind/readme.md) | Why a fee rate is a numerator/denominator ratio rather than a scaled conserved value — keeps multiply-before-divide explicit |

### Sources

| File | Relationship |
|------|--------------|
| `conversation/anatolii_shlyakhto/001_capstone_recruitment_thread.md` (design corpus) | The worked fill whose price-improvement rule is named here and deliberately not adopted |
| `ecs_system/054_fee_collection_system.md` (design corpus) | Assessment on executed deals, rate unspecified — the half this instance adopts |
| `faction/007_genesis_state.md` (design corpus) | The single 5% figure, recorded because it conflicts with the above rather than because it is adopted |
| `src/lib.rs` | The settle loop where the hook belongs; no fee is charged while the schedule is open |

### Tests

| File | Relationship |
|------|--------------|
| `tests/fee_assessment_test.rs` (to create) | Total fee over a fully-executed order is identical whether it filled in one part or many; no fee appears in any stream containing no `Trade`; every `Trade` names exactly one resting and one aggressing side; fee amounts summed across both sides plus the fee account balance exactly, with no residual |
