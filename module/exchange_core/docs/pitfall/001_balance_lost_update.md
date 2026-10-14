# Pitfall: Balance Lost Update Under Concurrent Settlement

### Scope

- **Purpose**: Name the one failure that leaves every order-level invariant intact and the ledger wrong, before somebody parallelizes settlement because matching is the hot path and settlement looks like the easy part to split.
- **Responsibility**: State the trap, the failure with its worked trace and why nothing errors, and the mitigation — plus why the conservation audit is not one.
- **In Scope**: Concurrent read-modify-write on an account balance from more than one settlement path, and what makes the resulting corruption silent and unattributable.
- **Out of Scope**: The partition being corrupted (→ [Available and Reserved Balance Separation](../invariant/003_available_and_reserved_balance_separation.md)); the merge mechanism the mitigation relies on, a separate contract; the audit's own algorithm (→ [Conservation Audit](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md)).

### Trap

**Read-modify-write on an account balance, from two matching paths at once.**
The trap is that the operation *looks* atomic when written — `balance -=
amount` is one line — and is three: load, compute, store. Another settlement
can land between the load and the store, and the second store overwrites the
first's result with a value computed from a stale read.

This crate is specifically exposed, for three reasons that compound:

- **The conflict set is not a rare pair.** A balance is shared by every order
  the account has resting, so any two concurrent fills touching one account
  conflict. Unlike most lost-update hazards, this one does not require an
  unusual access pattern; it *is* the access pattern.
- **Settlement is four writes, not one.** A fill moves quantity out of one
  account's `reserved`, into another's `available`, and mirrors both on the
  other asset (→ [Available and Reserved Balance Separation](../invariant/003_available_and_reserved_balance_separation.md))
  — the corpus names this general shape directly, as "ACID-safe movement of
  money" between two accounts (→ external design corpus
  `ecs_system/053_transactional_transfer_system.md`, plain-text citation),
  without naming the mechanism that makes it so. Each of the four is a
  read-modify-write, and any of the four can be the one that is lost —
  leaving a settlement that is half applied and whose halves are each
  individually plausible.
- **Matching is the hot path**, so it is precisely where somebody will
  eventually be tempted to parallelize, and settlement is the part that looks
  independent per account.

**The corpus records the trace, with numbers** (→ external design corpus
`review/002_concurrency_architecture_contradictions.md`, plain-text citation):
two writers each read a pre-state of 1000; one records a −100 result as
`SET 900`, the other a +200 result as `SET 1200`. Neither replay order reaches
the correct 1100. And the conclusion that generalizes it is the sentence worth
lifting verbatim — "determinism settles which value survives, never whether an
update is lost" (→ external design corpus
`decision/050_deferred_mutation_accumulator_scope.md`, plain-text citation).

That sentence is the trap's real shape. A deterministic engine is not a safe
one. A deterministic lost update is lost the same way on every replay, which
removes the one symptom — irreproducibility — that would otherwise expose it.

### Failure

**The classic lost update, and what makes it worse here is where it
surfaces.** The book's own accounting is untouched. Both orders filled, the
quantities matched, and the event stream records both trades correctly
(→ [Trade Event Stream](../protocol/001_trade_event_stream.md)). Only the
ledger is wrong, by exactly the overwritten delta. So every check that exists
passes:

- **Nothing errors at the moment of the fault.** Both settlements
  "succeeded"; neither saw an inconsistent value.
- **No order is in an illegal state**
  (→ [Order Lifecycle](../state_machine/001_order_lifecycle.md)), so no state
  assertion fires and no lifecycle test can be written that would.
- **Escrow coverage still holds for every resting order**
  (→ [Escrow Covers Resting Orders](../invariant/001_escrow_covers_resting_orders.md)),
  because the corrupted amount is one that was already released — the
  invariant is stated over orders that are still on the book, and this one is
  not.
- **Replay does not find it.** Effect-replay reconstructs the same wrong
  balance from the same correct events, because the corruption was never an
  event. Input-replay through a matcher that still contains the race
  reproduces the race.

**The discrepancy surfaces at the conservation audit**
(→ [Conservation Audit](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md))
as a sum that does not balance — with **no attribution**. The audit reports
that units were created or destroyed across a window containing an arbitrary
number of trades, and cannot say which one, which account, or when. Three
properties make that report nearly useless as a starting point:

- **Detection latency is unbounded.** Nothing forces the audit to run soon,
  and a corrupted balance is a perfectly legitimate input to every subsequent
  order — so the error propagates into later fills that are themselves
  correct given a wrong input. By the time the audit runs, the wrong balance
  has authored consequences that are not wrong and cannot be unwound.
- **The magnitude carries no signal.** The lost delta can be any size, so a
  small imbalance is not evidence of a small bug, and a large one is not
  evidence of a recent one.
- **The natural response destroys the evidence.** An audit that reports a
  drift invites reconciliation — writing the difference back so the next
  audit passes. That converts a detected corruption into an undetectable one,
  permanently, and it is the response an operator under time pressure will
  reach for.

**And it is not detectable by counting anything.** The same trades exist, the
same quantities matched, the same events were emitted. This is the ledger-side
instance of the same shape that recurs one layer down, in whatever merge
mechanism the host eventually supplies: the totals reconcile there too.

### Mitigation

**1. One writer — flows through, not protected by.** Every balance mutation
goes through the single-consumer merge path
(→ [Arrival Sequence Is Total and Replayable](../invariant/002_arrival_sequence_total_and_replayable.md)).
The distinction from "is protected by a lock" is the whole mitigation: a lock
is a discipline that a new call site can forget, while a single consumer is a
structure a second writer has to be given access to. A parallel system that
mutates a balance directly is **out of contract**, not a fast path — which is
this crate's own rule against building a second intake mechanism, per
`task/decisions.md` Q-07. The corpus
states the resulting property for exactly this engine: a single sequential
consumer applying discrete events in ring order is "accumulation-safe by
construction" (→ external design corpus
`decision/051_order_matching_engine_architecture_ruling.md`, plain-text
citation).

**2. Deltas rather than absolutes, wherever a second writer is even
possible.** The corpus's ruling for its own multi-writer case is that
accumulation-style writes use delta-typed records summed at merge, whose
order-independence follows from commutativity (→ external design corpus
`decision/050_deferred_mutation_accumulator_scope.md`, plain-text citation).
The matching path does not need it — one consumer means one writer — but it
is the required shape for anything *else* that credits or debits an account,
and adopting it there is what stops the matching path's guarantee from being
quietly bypassed by a system that never went near the match loop. **This is
where the pitfall can re-enter, and it is open:** whether deposits,
withdrawals, and any external settlement ride the same merge path or a
separate one is undecided, and only the same-path answer makes mitigation 1
complete. Note also that a delta record is signed while a holding is not, so
whichever answer is chosen has to state what an underflowing debit does —
the corpus reconciles the two nowhere.

**3. The audit is not a mitigation, and treating it as one is the second
trap.** The conservation audit exists *because* this class of bug is silent
at the moment it happens; it is a detector of last resort with no
attribution and unbounded latency. Structural prevention is what keeps its
failure rate at zero, and zero is the only rate at which its output is
actionable — an audit that fails occasionally and is reconciled by hand has
stopped being evidence and become a chore.

**4. What a test can actually establish**, since "no data race" is not
demonstrable by sampling:

- A property test driving N concurrent submitters against one account, then
  asserting after quiescence that `available + reserved` equals the deposited
  total **exactly**, with no tolerance
  (→ [Available and Reserved Balance Separation](../invariant/003_available_and_reserved_balance_separation.md)).
  No tolerance is the point: a tolerance is the reconciliation habit above,
  written into the test suite.
- An interleaving harness (`loom`- or `miri`-style) over any hand-written
  lock-free path, because the same caveat applies here regardless of the
  eventual merge mechanism: on a total-store-order machine the reordering
  that exposes the bug does not occur, so a green suite on x86-64 developer
  hardware is not evidence about a weakly-ordered deployment target.
- A deliberate negative test: a second writer wired directly to the balance,
  asserted to be rejected at the type or API level rather than merely
  discouraged in prose.

### Algorithms

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md) | The detector of last resort — what it reports, why it cannot attribute, and why it is not the mitigation |
| [../algorithm/001_price_time_priority_matching.md](../algorithm/001_price_time_priority_matching.md) | Steps 2, 4, and 5 are the balance mutations this trap sits on |
| [../algorithm/004_fee_assessment_point.md](../algorithm/004_fee_assessment_point.md) | A third mutation per fill, to the fee account — another read-modify-write with the same exposure |

### Features

| File | Relationship |
|------|--------------|
| [../feature/001_exchange_core_v0_1.md](../feature/001_exchange_core_v0_1.md) | Why the hosting commitment is load-bearing rather than convenient — this trap is what a second writer falls into silently |

### Invariants

| File | Relationship |
|------|--------------|
| [../invariant/001_escrow_covers_resting_orders.md](../invariant/001_escrow_covers_resting_orders.md) | Why coverage still passes over a corrupted ledger — it is stated over orders still on the book |
| [../invariant/002_arrival_sequence_total_and_replayable.md](../invariant/002_arrival_sequence_total_and_replayable.md) | The single path every mutation flows through, and why determinism alone does not save the ledger |
| [../invariant/003_available_and_reserved_balance_separation.md](../invariant/003_available_and_reserved_balance_separation.md) | The four fields at risk per settlement, and the exact assertion mitigation 4 tests |

### Protocols

| File | Relationship |
|------|--------------|
| [../protocol/001_trade_event_stream.md](../protocol/001_trade_event_stream.md) | Records both trades correctly while the ledger is wrong — why effect-replay reproduces the corruption faithfully |

### State Machines

| File | Relationship |
|------|--------------|
| [../state_machine/001_order_lifecycle.md](../state_machine/001_order_lifecycle.md) | No order reaches an illegal state, which is why no lifecycle assertion can catch this |

### Sources

| File | Relationship |
|------|--------------|
| `decision/050_deferred_mutation_accumulator_scope.md` (design corpus) | The delta-record ruling of mitigation 2, and the determinism-does-not-help sentence |
| `decision/051_order_matching_engine_architecture_ruling.md` (design corpus) | The accumulation-safe-by-construction property a single sequential consumer supplies |
| `ecs_system/053_transactional_transfer_system.md` (design corpus) | Names "ACID-safe movement of money" as its own term for the four-write shape this trap sits on |
| `review/002_concurrency_architecture_contradictions.md` (design corpus) | The 1000 / 900 / 1200 trace, and the finding that neither replay order reaches 1100 |
| `../../../exchange_escrow/src/lib.rs` | `settle` — the single path that moves value, so no caller can update one side of a balance alone |

### Tests

| File | Relationship |
|------|--------------|
| `tests/balance_concurrency_test.rs` (to create) | N concurrent submitters against one account: `available + reserved` equals the deposited total exactly after quiescence, with zero tolerance; a settlement path wired outside the merge is rejected rather than merely discouraged; an interleaving harness over any hand-written lock-free path, run on at least one weakly-ordered target |
