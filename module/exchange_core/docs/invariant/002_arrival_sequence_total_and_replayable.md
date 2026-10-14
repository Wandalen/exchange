# Invariant: Arrival Sequence Is Total and Replayable

### Scope

- **Purpose**: Give "time priority" something to be a priority *over*, and make every fill re-derivable from the record of what was accepted — the two properties matching and auditing both rest on and neither supplies.
- **Responsibility**: State the totality and replay-determinism clauses, the substrate mechanism enforcing them, the three obligations that mechanism does not cover, and what an irreproducible fill costs.
- **In Scope**: The order in which accepted orders enter matching, and what re-running that order against the same starting book must produce.
- **Out of Scope**: The merge mechanism itself, a separate concern with its own contract; which concurrency pattern wins (undecided); what matching does once ordered (→ [Price-Time Priority Matching](../algorithm/001_price_time_priority_matching.md)); the event record replay reads (→ [Trade Event Stream](../protocol/001_trade_event_stream.md)).

### Invariant Statement

Every accepted order occupies **exactly one position in a single total
arrival order**, and re-running the same accepted sequence against the same
starting book produces **byte-identical fills, in the same order, leaving the
same residual book**.

Three clauses, each independently load-bearing:

- **Totality.** For any two accepted orders A and B, exactly one of A ≺ B or
  B ≺ A holds. No ties, no partial order, no "same instant". Concurrency at
  submission does not survive acceptance.
- **Position is claimed, not measured.** An arrival position is a claimed
  sequence number, never a reading of a clock. Two submissions carrying equal
  host timestamps — or arriving from hosts whose clocks disagree — still
  receive distinct, ordered positions. A clock is an input the sequence does
  not carry, and every such input is a divergence source.
- **Replay determinism.** The fill sequence is a pure function of *(starting
  book, accepted sequence)*. Not of thread count, not of drain batching, not
  of how many drain calls the sequence was consumed across, not of allocation
  addresses or hash iteration order.

**This is what makes price-time priority meaningful at all.**
[Price-Time Priority Matching](../algorithm/001_price_time_priority_matching.md)
step 3 says "within one price, strictly oldest first, FIFO with no
exceptions" — but *oldest* is a predicate over an order this crate must
supply, and until it does, that clause names a total order it does not itself
construct. Two orders resting at the same price with no decided winner is not
a rare edge; under any concurrent intake it is the normal case for every
liquid price level.

### Enforcement Mechanism

**The mechanism is external, by ruling.** Per `task/decisions.md`
Q-07 this crate does not build its own intake, so all three clauses are
discharged by whatever single-consumer merge discipline the eventual
concurrency host provides, not re-derived here. That discipline needs to
supply:

- **Totality**, from a sequence number claimed atomically *before*
  publication. Uniqueness and monotonicity are properties of the
  read-modify-write's atomicity, not of any ordering it establishes.
  The serialized cost of one such claim is roughly **20 ns** against roughly
  **500 ns** of parallel payload work (→ external design corpus
  `concurrency_pattern/003_disruptor_pattern.md`, plain-text citation); that
  ≈25× ratio is the whole reason one serialization point can buy a total
  order without becoming the bottleneck.
- **Reproducible order**, from the drained order *being* sequence
  order — no post-drain sort, no per-producer sub-queues surfacing in a
  nondeterministic interleave.
- **Integrity of each position**, from a Release/Acquire pairing. A
  torn or lap-confused element delivers an order that was never submitted, at
  a position that was never claimed — breaking both the totality and the
  replay clauses at once while every element count still reconciles.

**Three obligations the substrate does not cover, which are this crate's:**

1. **Acceptance is the sequencing point that matters, not submission.** Every
   submitted order receives a merge position, but a rejected order changes no
   book state (→ [Order Lifecycle](../state_machine/001_order_lifecycle.md),
   Rejected). The replay clause is therefore stated over the *accepted*
   subsequence; rejections are still recorded, so the stream stays a complete
   audit rather than a filtered one
   (→ [Trade Event Stream](../protocol/001_trade_event_stream.md),
   `OrderRejected`).
2. **Replay must start from a named book state.** "Byte-identical fills" is
   meaningless without a stated starting point. v0.1 fixes exactly one:
   replay from empty
   (→ [Exchange Core v0.1](../feature/001_exchange_core_v0_1.md) exit
   criterion 4). Whether the crate additionally supports replay from a
   checkpoint — which requires the checkpoint to be byte-canonical, not
   merely equivalent — is **open**, and it is the same open question as
   whether an event stream is ever spliced
   (→ [Trade Event Stream](../protocol/001_trade_event_stream.md), Version
   Compatibility).
3. **No matching-path decision may consult an input the sequence does not
   carry.** No wall-clock read, no map iteration order, no pointer or slot
   address in a tie-break, no floating-point arithmetic anywhere in a
   comparison. This is a requirement on this crate's own code that no amount
   of substrate correctness can supply; a single address-dependent tie-break
   defeats clause three while every substrate test stays green.

**The arrival position is the sequence number of the order's `OrderAccepted`
event** (→ [Trade Event Stream](../protocol/001_trade_event_stream.md)), so
arrival positions are strictly increasing but deliberately **not
contiguous** — the same counter also numbers rejections, trades, and
cancels. A total order needs strictness, never contiguity, and identifying
the two counters removes a second thing that could disagree.

### Violation Consequences

**An irreproducible fill is an unauditable fill.** Dispute resolution,
post-hoc audit, and the crate's own recovery story all consist of re-deriving
state from the record. A fill the record does not reproduce is a fill nobody
can check — and the divergence does not surface at the fill that caused it.
It surfaces at the first later order whose match *depended* on the
difference, arbitrarily far downstream, as a rebuilt book that disagrees with
the live one and no way to know which of the two is right.

**Without totality, price-time priority silently degrades to price priority
alone.** The winner between two same-price orders becomes whichever the merge
happened to hand over first: stable on one machine, different on another, and
indistinguishable from correct behaviour in every single-threaded test — the
regime in which matching engines are almost always tested.

**Zero-sum conservation survives the violation, which is what makes it
dangerous.** A wrong-winner fill moves the correct total quantity between the
wrong pair of accounts. The conservation audit
(→ [Conservation Audit](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md))
balances exactly, every element is accounted for, and the only observable
symptom is that one participant was filled who should not have been — a
complaint with no evidence behind it. The same shape recurs one layer down,
in whatever merge mechanism the host eventually supplies: the totals
reconcile regardless.

**And the recovery story collapses rather than degrades.** Event-sourced
recovery is not a feature that gets less accurate under this violation; it
stops being a source of truth at all, because a rebuilt book that disagrees
with the live book supplies no repair path. There is no third record to
break the tie.

### Algorithms

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md) | Balances regardless of which of two same-price orders won — why this failure class is invisible to it |
| [../algorithm/001_price_time_priority_matching.md](../algorithm/001_price_time_priority_matching.md) | Step 3's "strictly oldest first" is a predicate over this order, and undefined without it |
| [../algorithm/003_cancel_and_amend_semantics.md](../algorithm/003_cancel_and_amend_semantics.md) | Cancel-versus-fill and amend-versus-fill are decided by position in this same order, not by timing |

### Features

| File | Relationship |
|------|--------------|
| [../feature/001_exchange_core_v0_1.md](../feature/001_exchange_core_v0_1.md) | Exit criterion 4 is this invariant under cold replay from empty |

### Invariants

| File | Relationship |
|------|--------------|
| [003_available_and_reserved_balance_separation.md](003_available_and_reserved_balance_separation.md) | The balance partition mutated once per position, which is what makes its conservation clause replayable |

### Pitfalls

| File | Relationship |
|------|--------------|
| [../pitfall/001_balance_lost_update.md](../pitfall/001_balance_lost_update.md) | The mutation path this order is the single writer for; a second writer breaks the ledger without breaking any count |

### Protocols

| File | Relationship |
|------|--------------|
| [../protocol/001_trade_event_stream.md](../protocol/001_trade_event_stream.md) | Carries this order as its `sequence` field, and distinguishes effect-replay from the input-replay this invariant states |

### State Machines

| File | Relationship |
|------|--------------|
| [../state_machine/001_order_lifecycle.md](../state_machine/001_order_lifecycle.md) | BI7 — every transition happens at exactly one arrival position, which is what makes "at every observable point" well-defined |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs` | `claim_order`/`emit` — the single-threaded sequence counter, never a clock read |
| `../../../exchange_book/src/lib.rs` | Carries `Sequence` as claimed arrival position; ranks by it, never by insertion time |
| `../../../exchange_match/src/lib.rs` | The match loop, a pure function of book and order — no clock, hash order, or address |

### Tests

| File | Relationship |
|------|--------------|
| `tests/arrival_order_test.rs` (to create) | Under N-thread submission, two orders never share a position; the same accepted sequence replayed against an empty book yields identical fills across runs and across drain batchings; a matching path that reads a clock or a hash iteration order fails a deliberate shuffle-the-container test |
