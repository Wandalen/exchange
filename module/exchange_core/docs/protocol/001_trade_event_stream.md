# Protocol: Trade Event Stream

### Scope

- **Purpose**: Fix what a consumer of the engine is entitled to observe and depend on, before any consumer exists — because a stream that is already archived cannot be renegotiated, and recovery, audit, and every downstream analytic read the same bytes.
- **Responsibility**: Define the fields common to every event, the five event kinds and their distinguishing payloads, and the compatibility rules an added field or an added kind must obey.
- **In Scope**: The *message* contract — which events exist, what each one asserts, and what a replaying consumer may rely on across versions.
- **Out of Scope**: The on-wire encoding, which is opaque to whatever carries this stream as payload; the on-disk encoding of a conserved value (no successor doc — the old `exact_arithmetic` crate's transaction-log-encoding design was never carried into the real 16-crate [exact](https://github.com/Wandalen/exact) family); the *input* side, which is a separate, upstream intent-submission protocol this crate does not define.

### Abstract

The event stream is the engine's only output besides the book itself, and the
only one that persists. It is append-only, totally ordered, and complete:
**no state change occurs without its event**
(→ [Price-Time Priority Matching](../algorithm/001_price_time_priority_matching.md)
step 6), which is what makes replaying it a reconstruction rather than an
approximation.

**This document fixes messages, not bytes.** Nothing here states a width, an
endianness, an offset, or a framing rule. Two other crates own those and
neither is this one: the transport carries the stream as payload it never
parses, and the durable log encodes conserved values under its own format.
This crate fixes only what a consumer is *entitled to observe* — which is the
part that cannot be changed later, because it is the part consumers write
code against.

**Effect, not intent — the deliberate opposite of a host's input log.** An
input log records what a caller *asked for*, so replaying it re-runs the logic
that decided what happened. This stream records what *happened*: a `Trade` is
not a request to trade. The difference is not stylistic, and both replays are
real:

- **Effect-replay** — apply this stream's events to an empty book and ledger.
  Reproduces book and ledger state without re-running matching. This is
  [Exchange Core v0.1](../feature/001_exchange_core_v0_1.md) exit criterion 4.
- **Input-replay** — feed the accepted order sequence back through the match
  loop against the same starting book. Reproduces the fills themselves,
  which is what makes a disputed fill checkable rather than merely restorable
  (→ [Arrival Sequence Is Total and Replayable](../invariant/002_arrival_sequence_total_and_replayable.md)).

The two must agree. Effect-replay is cheap and cannot detect a matching bug;
input-replay is expensive and can. Naming them separately is what stops
"replay reproduces state" from being read as "replay proves the fills were
right".

### Message Structure

Four fields are common to every event, whatever its kind:

| Field | Meaning | Consumer may rely on |
|-------|---------|----------------------|
| `sequence` | Position in the total order. Numbers **every** event, strictly increasing by one. | Gap-freedom: a consumer detects loss by arithmetic alone, with no acknowledgement protocol and no heartbeat. |
| `timestamp` | A host clock reading, attached for humans and external analytics. | Nothing. Two events may carry equal timestamps and clocks may disagree between hosts; `sequence` breaks every tie. |
| `order_id` | The order this event concerns. For `Trade`, the **aggressing** order. | Stability: an order's id is fixed at submission and never reassigned, including across an amend. |
| `account_id` | The account owning `order_id`. | Attribution without a lookup: a consumer never needs an order table to know whose event this is. |

**`timestamp` is descriptive and never load-bearing.** No engine decision may
read it, and a consumer that reconstructs state from it rather than from
`sequence` reconstructs a different state on a different host. A clock
reading is an effect of the machine, not a fact about the world being
recorded.

**A `Trade` has two sides, and the common fields name the aggressor.** The
resting side travels in the payload. This is not a compression trick — it is
what makes the maker/taker classification a property of the envelope rather
than a computation each consumer re-derives, and re-derives differently
(→ [Fee Assessment Point](../algorithm/004_fee_assessment_point.md)).

**An order's arrival position is the `sequence` of its `OrderAccepted`
event.** Arrival positions are therefore strictly increasing but deliberately
**not contiguous**, since the same counter also numbers rejections, trades,
cancels, and amends. A total order needs strictness, never contiguity, and
identifying the two counters removes a second thing that could disagree
(→ [Arrival Sequence Is Total and Replayable](../invariant/002_arrival_sequence_total_and_replayable.md)).

Every quantity, price, reserved amount, released amount, and fee below is an
[`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_kind/readme.md)
conserved value. This crate names the field; it never states its scale, its
range, or how it rounds.

### Message Types

| Kind | Emitted when | Distinguishing payload | Effect on book and ledger |
|------|--------------|------------------------|---------------------------|
| `OrderAccepted` | Validation and reservation both succeed, before any matching | Side, order type, limit price (absent for a market order), submitted quantity, Time-in-Force, self-match key, reserved amount | Quantity moved `available → reserved`; the order becomes visible to matching. Its `sequence` is the order's arrival position |
| `OrderRejected` | Validation fails, or the reservation cannot be taken in full | Rejection reason, from a **closed** set | None. No book state ever existed and no units were reserved |
| `Trade` | The match loop generates one fill | Resting order id, resting account id, executed price, executed quantity, and the fee assessed against each side | Both orders' remaining quantities reduced by the executed quantity; both reservations reduced by their matched portions; the units transfer |
| `OrderCancelled` | A remainder is withdrawn | Cause — request, Time-in-Force disposition, or self-match — plus cancelled quantity and released amount | The order leaves the book; the remainder's reservation returns to `available` |
| `OrderAmended` | An amend is applied in full | Which fields changed, new price and/or new quantity, whether time priority was **retained or forfeited**, and the escrow delta | The order's book position and reservation are updated in one step; its arrival position may be reassigned |

**The rejection reason is a closed set, not a string.** Which reasons exist
is open and will grow; that a consumer can *branch* on the reason rather than
parse prose is decided now, because it is the part that cannot be added
later without every existing consumer's error handling being wrong.

**Priority retention is a stored field, not a re-derived rule.** A consumer
reading an archived stream must not have to know which version of
[Cancel and Amend Semantics](../algorithm/003_cancel_and_amend_semantics.md)
was in force when the amend was applied. Recording the outcome keeps old
streams interpretable when the rule changes; re-deriving it makes every rule
change retroactively rewrite history.

**There is deliberately no `OrderFilled` kind.** An order is Filled when the
sum of its `Trade` quantities equals its submitted quantity — a fact the
`Trade` events already carry exactly
(→ [Order Lifecycle](../state_machine/001_order_lifecycle.md) BI4). A
dedicated terminal event would be a second record of one fact, and a consumer
that trusted it would diverge from one that computed it, silently, in exactly
the cases where the two disagree. The same argument forbids a separate fee
event: the fee is a field of the `Trade` that caused it, so a consumer cannot
apply it twice
(→ [Fee Assessment Point](../algorithm/004_fee_assessment_point.md)).

**Open — no snapshot or checkpoint kind is defined.** v0.1 replays from
empty. A checkpoint kind would have to be byte-canonical rather than merely
equivalent, or two checkpoints of the same state would produce two streams,
and it is the same open question as whether a stream is ever spliced (below).

### Version Compatibility

**Additive-field-only evolution.** A later version may add fields to an
existing kind and may add new kinds. It may never remove a field, change a
field's meaning, change a field's type, or renumber a kind.

**Unknown *field* → skip and process. Unknown *kind* → hard failure.** The
asymmetry is deliberate: an unknown field on a known kind is additional detail
about a transition the consumer already understands, so skipping it loses
annotation. An unknown kind is a state transition the consumer does not
understand, and every subsequent event assumes it was applied — skipping one
produces a book that is confidently wrong, which is worse than refusing to
load.

**The consequence, stated rather than discovered later: adding a kind is a
breaking change; adding a field is not.** A new kind must therefore be
introduced with a version bump that old replaying consumers can refuse on,
not slipped in as an extension.

**No added field may be load-bearing for state reconstruction.** This is the
teeth of "additive only". If reaching correct book state from a v2 stream
requires a field v1 did not have, then every v1 stream is retroactively
unreplayable and the whole audit story is void for all history before the
change. So every added field is descriptive, or redundant with fields already
present — never the sole carrier of an effect.

**The replay rule this all serves:** a consumer replaying an older stream
must reach the same book and ledger state a consumer of that era reached.
Not a *compatible* state, not an *upgraded* one — the same one. A migration
that "improves" historical state is indistinguishable, from the outside, from
a bug that corrupts it.

**Open — where the version marker lives.** Per-stream, at the head, is
cheaper and sufficient if a stream is never spliced. Per-event survives
splicing, at a per-event cost. Whether streams are ever spliced — a
checkpoint followed by a tail — is the same open question as checkpointing
above and in
[Arrival Sequence Is Total and Replayable](../invariant/002_arrival_sequence_total_and_replayable.md);
deciding either decides both.

### Algorithms

| File | Relationship |
|------|--------------|
| [../algorithm/001_price_time_priority_matching.md](../algorithm/001_price_time_priority_matching.md) | Step 6 emits every event here; its open executed-price rule is what `Trade`'s executed price will carry |
| [../algorithm/002_self_match_prevention.md](../algorithm/002_self_match_prevention.md) | Supplies `OrderCancelled`'s self-match cause and `OrderAccepted`'s self-match key |
| [../algorithm/003_cancel_and_amend_semantics.md](../algorithm/003_cancel_and_amend_semantics.md) | Defines `OrderAmended`'s priority-retention outcome and `OrderCancelled`'s request cause |
| [../algorithm/004_fee_assessment_point.md](../algorithm/004_fee_assessment_point.md) | Why the fee is a `Trade` field rather than a kind of its own |

### Features

| File | Relationship |
|------|--------------|
| [../feature/001_exchange_core_v0_1.md](../feature/001_exchange_core_v0_1.md) | Exit criterion 4 is effect-replay of this stream from empty |

### Formats

| File | Relationship |
|------|--------------|
| *(no successor — see note)* | The on-disk encoding of every conserved value named as a field here; the old `exact_arithmetic` crate's transaction-log-encoding design this once cited was never carried into the real 16-crate [exact](https://github.com/Wandalen/exact) family |

### Invariants

| File | Relationship |
|------|--------------|
| [../invariant/001_escrow_covers_resting_orders.md](../invariant/001_escrow_covers_resting_orders.md) | Why the release travels inside the fill and cancel events rather than in a later settlement event |
| [../invariant/002_arrival_sequence_total_and_replayable.md](../invariant/002_arrival_sequence_total_and_replayable.md) | The order `sequence` carries, and the input-replay this stream's effect-replay is distinguished from |
| [../invariant/003_available_and_reserved_balance_separation.md](../invariant/003_available_and_reserved_balance_separation.md) | The partition reserved and released amounts move quantity across, reconstructed rather than recomputed |

### Pitfalls

| File | Relationship |
|------|--------------|
| [../pitfall/001_balance_lost_update.md](../pitfall/001_balance_lost_update.md) | Records both trades correctly while the ledger is wrong — this trap is why effect-replay reproduces the corruption faithfully rather than exposing it |

### State Machines

| File | Relationship |
|------|--------------|
| [../state_machine/001_order_lifecycle.md](../state_machine/001_order_lifecycle.md) | BI6 — every transition is carried by exactly one event here, which is why no `OrderFilled` kind is needed |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs` | `emit` — the sole event writer; `postings` derives the audit log from the stream |
| `../../../exchange_fill/src/lib.rs` | `Event`/`EventKind` — the message shapes this protocol fixes |

### Tests

| File | Relationship |
|------|--------------|
| `tests/event_stream_test.rs` (to create) | `sequence` is gap-free across a randomized session; effect-replay from empty and input-replay through the matcher reach identical book and ledger state; a stream carrying an unknown field replays to the same state, and one carrying an unknown kind refuses to load rather than skipping |
