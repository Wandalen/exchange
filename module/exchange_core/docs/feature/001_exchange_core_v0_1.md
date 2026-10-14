# Feature: Exchange Core v0.1

### Scope

- **Purpose**: Fix what the first real version of the matching engine covers, so scope lives here and mechanics live in the owning definitions.
- **Responsibility**: State version scope, deliberate exclusions, and exit criteria, with pointers to the artifacts realising each commitment.
- **In Scope**: Order and matching semantics this version commits to, hosting and value-type decisions already ruled, and when v0.1 is done.
- **Out of Scope**: Matching implementation detail (spike-first, → [`spike/readme.md`](../../../../spike/readme.md)); the intake concurrency mechanism, decided elsewhere; game-side market content.

### Design

The engine's whole contract in one line: **orders
in → trades + event stream out; escrow holds.** Order books match buy against
sell under price-time priority, with every value an exact type.

**Commitments** — each owned by exactly one artifact this doc points at
rather than restates:

- **Order semantics.** Market, Limit, and Stop orders; Time-in-Force
  handling for FOK, IOC, and GTC; partial fills that split
  conservation-exact. Matching order is price priority, then time priority
  within a price level (→ [Price-Time Priority Matching](../algorithm/001_price_time_priority_matching.md)).
- **Escrow.** Reservation before an order rests; instant release on fill or
  cancel (→ [Escrow Covers Resting Orders](../invariant/001_escrow_covers_resting_orders.md)).
- **Values.** Every price, quantity, and balance is an
  [`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md)
  type — the float ban is upstream of this crate, not re-argued here.
- **Hosting.** No concurrency host is integrated yet; intake will ride
  whatever mechanism the eventual host provides, never a bespoke one built
  inside this crate.
- **Recovery.** Event-sourced: every book and ledger state change is an
  emitted event, and replaying the event stream reproduces the state.

**Since designed, and now owned elsewhere** — two of the three items the
version's scope-listed without semantics have them:

- **Fee computation** — the assessment *point* and the maker/taker
  *classification* are decided; the rate and its schedule are deliberately
  not (→ [Fee Assessment Point](../algorithm/004_fee_assessment_point.md)).
- **Wash-trading safeguards** — self-crossing never trades, and the three
  cancel policies are enumerated with their leakage and denial-of-service
  surfaces; which one is the default stays open
  (→ [Self-Match Prevention](../algorithm/002_self_match_prevention.md)).

**Still named but not designed:** slippage safeguards. Left TBD rather than
invented — the source material names the requirement and simultaneously
pushes back on it, so there is nothing here to transcribe and no measurement
to decide it against.

**Specified ahead of its version slot:** amend. Neither the commitments above
nor the version's scope list it, and no amend operation exists in the source
material at all — but its priority rule is decided now, in
[Cancel and Amend Semantics](../algorithm/003_cancel_and_amend_semantics.md),
so that adding the operation is an increment rather than a redesign. Which
version carries it is not fixed here.

**Deliberate exclusions:**

- Any UI or market presentation — game-side entirely.
- Derivatives and complex finance — the corpus builds them on this engine
  later; v0.1 is the spot engine only.
- Client-side prediction — schema-bound, out of scope entirely.
- Its own concurrency machinery — forbidden by the rulebook rule above;
  the engine consumes the substrate's mechanism.
- Physical settlement logistics beyond the escrow hook — cargo-side
  delivery is game content; this crate stops at the reservation/release
  events.

**Exit criteria** — v0.1 is done when all four hold:

1. Orders in → trades + events out, for all three order types across all
   three TIF modes, partial fills included.
2. The conservation audit over the emitted event stream balances exactly
   (→ [Conservation Audit](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md)).
3. The escrow invariant holds at every observable point under a randomized
   order/cancel/fill stream.
4. Cold recovery: replaying the event stream from empty reproduces
   byte-identical book and ledger state.

### Algorithms

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md) | Exit criterion 2 — the audit this version's emitted event stream must balance under |
| [../algorithm/001_price_time_priority_matching.md](../algorithm/001_price_time_priority_matching.md) | The matching semantics commitment 1 points at |
| [../algorithm/002_self_match_prevention.md](../algorithm/002_self_match_prevention.md) | The wash-trading safeguard, moved from "named" to "designed" — ban fixed, policy configurable, default open |
| [../algorithm/003_cancel_and_amend_semantics.md](../algorithm/003_cancel_and_amend_semantics.md) | Specified ahead of its version slot; its cancel half already applies to this version's GTC orders |
| [../algorithm/004_fee_assessment_point.md](../algorithm/004_fee_assessment_point.md) | The fee assessment point and maker/taker classification; no rate |

### Features

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md) | The value-type substrate every price and balance uses |

### Invariants

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_minor/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_minor/readme.md) | The upstream float ban this crate's value commitment inherits |
| [../invariant/001_escrow_covers_resting_orders.md](../invariant/001_escrow_covers_resting_orders.md) | The escrow commitment, stated as a property with enforcement and violation cost |
| [../invariant/002_arrival_sequence_total_and_replayable.md](../invariant/002_arrival_sequence_total_and_replayable.md) | What the hosting commitment buys: the total order price-time priority is stated over, and exit criterion 4's replay |
| [../invariant/003_available_and_reserved_balance_separation.md](../invariant/003_available_and_reserved_balance_separation.md) | The storage shape the escrow commitment reserves into and releases from |

### Pitfalls

| File | Relationship |
|------|--------------|
| [../pitfall/001_balance_lost_update.md](../pitfall/001_balance_lost_update.md) | Why the hosting commitment is load-bearing rather than convenient — a second writer corrupts the ledger silently |

### Protocols

| File | Relationship |
|------|--------------|
| [../protocol/001_trade_event_stream.md](../protocol/001_trade_event_stream.md) | The recovery commitment's actual contract, and the stream exit criterion 4 replays |

### State Machines

| File | Relationship |
|------|--------------|
| [../state_machine/001_order_lifecycle.md](../state_machine/001_order_lifecycle.md) | The states exit criterion 1's order-type × TIF matrix walks |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs` | `Exchange` — this scope implemented, less TIF disposition, market orders, self-match, amend and fees |

### Tests

| File | Relationship |
|------|--------------|
| `tests/` (to create) | One suite per exit criterion — matching matrix, audit balance, escrow property under randomized streams, replay identity |
