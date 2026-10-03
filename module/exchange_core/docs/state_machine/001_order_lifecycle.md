# State Machine: Order Lifecycle

### Scope

- **Purpose**: Fix when an order is on the book, when it holds escrow, and when it is finished — three questions every matching step, every balance move, and every event consumer depends on, and which no other instance answers together.
- **Responsibility**: Enumerate the states, the legal transitions with their triggers, and the behavioural invariants no transition may break.
- **In Scope**: The lifecycle of one order from submission to a terminal state, including the self-transitions a partial fill and an amend produce.
- **Out of Scope**: The matching steps driving the transitions (→ [Price-Time Priority Matching](../algorithm/001_price_time_priority_matching.md)); the escrow coverage property the escrow column reports (→ [Escrow Covers Resting Orders](../invariant/001_escrow_covers_resting_orders.md)); the event each transition is carried by (→ [Trade Event Stream](../protocol/001_trade_event_stream.md)); the book structure an on-book order sits in (spike-first, → [`spike/readme.md`](../../../../spike/readme.md)).

### States

| State | On book | Escrow held | Terminal | Meaning |
|-------|:---:|:---:|:---:|---------|
| **Submitted** | no | no | no | Has an arrival position; not yet validated or reserved. Exists for exactly the span of matching steps 1–2. |
| **Accepted** | yes | yes | no | Validated, fully reserved, visible to matching, zero fills so far. Also read as *Resting*. |
| **PartiallyFilled** | yes | yes | no | At least one fill; remainder > 0. On the book at its original limit price with reduced remaining quantity and a reservation reduced by exactly the filled portion. |
| **Filled** | no | no | yes | Remaining quantity is zero. Every reserved unit has been transferred to counterparties. |
| **Cancelled** | no | no | yes | Remainder withdrawn — by request, by Time-in-Force disposition, or by self-match policy. The remainder's reservation was returned in the cancel event. |
| **Rejected** | no | no | yes | Failed validation or failed reservation. No book state ever existed and no units were ever reserved. |

**On-book is a property observable only *between* arrival positions.** An
IOC or FOK order can pass through PartiallyFilled and reach Cancelled inside
the processing of its own arrival position, so no other order ever observes
it resting. The column above says whether a state is on the book *if it
persists*, not that every occupant of that state is externally visible — and
that distinction is what keeps
[Escrow Covers Resting Orders](../invariant/001_escrow_covers_resting_orders.md)'s
"at every observable point" well-defined rather than requiring a rule for
sub-position instants.

**Accepted and PartiallyFilled are exactly the escrow-holding states, and
exactly the on-book states.** The two columns agree on every row, which is
BI5 below, and it is the reason coverage can be stated over "resting orders"
without separately enumerating which states rest.

**Rejected is the only terminal state that releases nothing**, because
nothing was ever reserved. That is what makes "escrow is discharged exactly
once on reaching a terminal state" a claim about two of the three terminal
states rather than all three — a release path attached to Rejected would
credit units nobody debited.

**Expired is a named open extension, not a state.** No committed
Time-in-Force has a time trigger: `{ FOK, IOC, GTC }` either resolve inside
one arrival position or rest indefinitely
(→ [Price-Time Priority Matching](../algorithm/001_price_time_priority_matching.md)
step 5), so there is no event for an Expired transition to fire on. Adding a
time-bounded Time-in-Force — good-till-date, good-till-time, session — is
what creates one, and it changes the shape of the problem rather than adding
a row: expiry is a scheduling question, so the trigger belongs to a calendar
substrate this crate does not define, rather than to the matching path, and
the cancel-versus-fill race it inherits is the one
[Cancel and Amend Semantics](../algorithm/003_cancel_and_amend_semantics.md)
already decides. Named here so the addition is an extension rather than a
redesign; **the Time-in-Force set itself stays open**, and this instance does
not extend it.

### Transitions

| From | To | Trigger | Notes |
|------|-----|---------|-------|
| — | Submitted | The order takes a position in the merged arrival order | The position is assigned before validation, so even a rejection is ordered (→ [Arrival Sequence Is Total and Replayable](../invariant/002_arrival_sequence_total_and_replayable.md)) |
| Submitted | Rejected | Validation fails, or the reservation cannot be taken in full | Matching steps 1–2. No escrow was taken, so nothing is released |
| Submitted | Accepted | Reservation succeeds and no opposite order crosses | The GTC resting case |
| Submitted | PartiallyFilled | Reservation succeeds; the match loop fills part of the quantity | |
| Submitted | Filled | Reservation succeeds; the match loop fills the whole quantity | Reachable in one position for a marketable order |
| Submitted | Cancelled | FOK pre-check fails, or an IOC/FOK finds no crossing liquidity at all | Whole reservation released in the cancel event |
| Accepted | PartiallyFilled | An incoming order consumes less than the remaining quantity | |
| Accepted | Filled | An incoming order consumes the whole remaining quantity | |
| Accepted | Cancelled | A cancel wins the race against a fill; or self-match policy cancels this side | Cause is carried in the event (→ [Trade Event Stream](../protocol/001_trade_event_stream.md)) |
| Accepted | Accepted | An amend is applied and the order still has no fills | Arrival position may be reassigned; priority is not a state (→ [Cancel and Amend Semantics](../algorithm/003_cancel_and_amend_semantics.md)) |
| PartiallyFilled | PartiallyFilled | A further fill leaves remainder > 0; or an amend is applied | Self-transition. A fill reduces the reservation by exactly the filled quantity, every time |
| PartiallyFilled | Filled | A fill consumes the remainder exactly | |
| PartiallyFilled | Cancelled | The remainder is cancelled by request, by IOC disposition, or by self-match policy | |

**There is no edge out of Filled, Cancelled, or Rejected.** A cancel or an
amend naming an order in one of those states is a *race result*, not a
caller error: it returns a defined, reportable outcome and changes nothing
(BI1).

**Amend is a self-transition on purpose.** An amend can move an order's
queue position, its price, and its reserved amount, and it changes no state —
because priority is not a state and never was. Modelling a price change as a
Cancelled → Accepted pair would be wrong in a way that matters: it would
imply an instant in which the order held no escrow and was off the book,
which is exactly the window
[Cancel and Amend Semantics](../algorithm/003_cancel_and_amend_semantics.md)
forbids.

### Behavioral Invariants

**BI1 — Terminal states are absorbing, and losing the race is not an
error.** Filled, Cancelled, and Rejected have no outgoing edges. A cancel or
amend against a terminal order returns a defined non-error result naming what
already happened — never a panic, never a state change, never a second
release. Losing the race is routine, not exceptional, so a mechanism that
treats it as a caller mistake puts a crash on the caller's ordinary path at
exactly the frequency the race occurs. This is the same cancel corollary a
calendar-driven scheduler would need for an expiring registration, carried
over unchanged because the argument is about the race, not about what is
being cancelled.

**BI2 — Escrow is discharged exactly once, on the transition into a terminal
state.** Reserved reaches zero exactly once per order: by transfer to
counterparties (Filled), by return to `available` (Cancelled), or vacuously
(Rejected, which reserved nothing). Not before — an order at rest is covered
for its full remaining obligation. Not after — the release is carried by the
same event as the transition, never by a later settlement pass. And never
twice: a second release credits units nobody debited, which is a mint
(→ [Available and Reserved Balance Separation](../invariant/003_available_and_reserved_balance_separation.md)).

**BI3 — A partial fill leaves the order on the book, at its original price,
with reduced remaining quantity.** The reservation is reduced by exactly the
filled portion in the same event. Neither the price nor the arrival position
changes — which is the same rule
[Cancel and Amend Semantics](../algorithm/003_cancel_and_amend_semantics.md)
states for an amending quantity decrease, for the same reason: shrinking a
claim never acquires queue position.

**BI4 — The sum of fills never exceeds the submitted quantity.** At every
state, `remaining == submitted − Σ fills`, exactly. With exact types that
equality is literal rather than approximate — there is no rounding slack for
a residual to hide in
(→ [Conservation-Exact Split](https://github.com/Wandalen/exact/blob/master/module/exact_dust/readme.md)).
`remaining` is never negative, and reaching zero is what *defines* Filled
rather than a separate condition that could disagree with it.

**BI5 — An order is on the book if and only if it holds escrow.** The two
boolean columns above are the same column. This is
[Escrow Covers Resting Orders](../invariant/001_escrow_covers_resting_orders.md)
stated as a state property: never under-covered means no on-book state
without escrow; never over-held means no escrow-holding state off the book.

**BI6 — Every transition is carried by exactly one emitted event — not
necessarily a dedicated one.** The Filled transition is carried by the Trade
that consumed the remainder; there is deliberately no `OrderFilled` kind
(→ [Trade Event Stream](../protocol/001_trade_event_stream.md)). What the
invariant forbids is a transition with *no* event and a transition with
*two*: the first makes the state unreconstructable from the stream, the
second gives a replaying consumer two chances to disagree with itself.

**BI7 — An order is in exactly one state at a time, and every transition
happens at exactly one arrival position.** No state exists between two
positions that is not in the table above, and no transition straddles two.
This is what makes the phrase "at every observable point" — used by the
escrow invariant and by every property test written against it — denote a
finite, enumerable set of instants rather than an appeal to wall-clock time.

### Algorithms

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_dust/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_dust/readme.md) | Why BI4's equality is exact rather than approximate across a partial-fill sequence |
| [../algorithm/001_price_time_priority_matching.md](../algorithm/001_price_time_priority_matching.md) | Drives every transition: steps 1–2 leave Submitted, step 3 produces the fill edges, step 5 produces the Time-in-Force cancel edges |
| [../algorithm/002_self_match_prevention.md](../algorithm/002_self_match_prevention.md) | Supplies the self-match cause on the Accepted → Cancelled and PartiallyFilled → Cancelled edges |
| [../algorithm/003_cancel_and_amend_semantics.md](../algorithm/003_cancel_and_amend_semantics.md) | Supplies the cancel edges and the two amend self-transitions, and decides the race BI1 declares non-fatal |
| [../algorithm/004_fee_assessment_point.md](../algorithm/004_fee_assessment_point.md) | BI4 — every executed unit belongs to exactly one fill, which is what makes this point's total fee invariant under splitting |

### Features

| File | Relationship |
|------|--------------|
| [../feature/001_exchange_core_v0_1.md](../feature/001_exchange_core_v0_1.md) | Commits the order types and Time-in-Force set these states are reachable under; exit criterion 1 walks the whole table |

### Invariants

| File | Relationship |
|------|--------------|
| [../invariant/001_escrow_covers_resting_orders.md](../invariant/001_escrow_covers_resting_orders.md) | BI5 is its coverage property as a state property; BI2 is its release-in-event clause |
| [../invariant/002_arrival_sequence_total_and_replayable.md](../invariant/002_arrival_sequence_total_and_replayable.md) | BI7's positions; a replay reconstructs every order's path through this table |
| [../invariant/003_available_and_reserved_balance_separation.md](../invariant/003_available_and_reserved_balance_separation.md) | The pair BI2's discharge moves quantity across, and the mint a double release would produce |

### Pitfalls

| File | Relationship |
|------|--------------|
| [../pitfall/001_balance_lost_update.md](../pitfall/001_balance_lost_update.md) | No order reaches an illegal state under this trap, which is why no lifecycle assertion here can catch it |

### Protocols

| File | Relationship |
|------|--------------|
| [../protocol/001_trade_event_stream.md](../protocol/001_trade_event_stream.md) | The events BI6 requires; its deliberate absence of an `OrderFilled` kind is why BI6 says *carried by* rather than *emits* |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs` | The transitions, as emitted events; no state field is stored separately from the stream |

### Tests

| File | Relationship |
|------|--------------|
| `tests/order_lifecycle_test.rs` (to create) | No transition leaves a terminal state; cancel and amend against each terminal state return a non-error result and change nothing; total released plus total transferred equals total reserved, once, per order; `remaining == submitted − Σ fills` after every event across randomized partial-fill sequences |
