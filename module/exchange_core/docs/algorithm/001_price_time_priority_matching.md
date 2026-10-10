# Algorithm: Price-Time Priority Matching

### Scope

- **Purpose**: Fix the matching semantics the engine commits to, so every implementation the spike tries is judged against one behaviour rather than each redefining it.
- **Responsibility**: State the decided steps of a single order's processing and name explicitly which representation choices stay open.
- **In Scope**: Validation, escrow, the match loop's ordering rule, exact fill splitting, TIF disposition, and event emission.
- **Out of Scope**: Book and price-level data structures (spike-first); the escrow property as a property (→ [Escrow Covers Resting Orders](../invariant/001_escrow_covers_resting_orders.md)); intake concurrency, decided elsewhere.

### Abstract

Processing one incoming order is a short, strictly ordered pipeline:
validate, reserve, match against the opposite book side in price-time
priority, split any partial fill exactly, dispose of the remainder per its
Time-in-Force, and emit events for every state change. The semantics below
are decided by this crate's own design and the corpus rulings cited inline
below. The two representation questions this
instance originally left open — the executed-price rule and the book's own
structure — were closed by the 2026-08-30 implementation and are recorded at the
end with their reasoning; what remains open is named there too, rather than
silently absorbed.

### Algorithm

1. **Validate.** Reject malformed orders whole: unknown type, unknown TIF,
   non-positive quantity, or any value field that is not an exact type.
2. **Reserve escrow.** Reserve the order's maximum settlement obligation
   before it can match or rest; rejection on insufficient funds happens
   here, atomically, with nothing yet visible to the book
   (→ [Escrow Covers Resting Orders](../invariant/001_escrow_covers_resting_orders.md)).
   For **FOK**, additionally pre-check full fillability against the visible
   opposite side — an FOK order either fills completely in step 3 or
   cancels completely in step 5; it never partially executes. Resting
   quantity carrying the incoming order's own self-match key is **excluded**
   from that visible side, since step 3 will refuse to trade against it
   (→ [Self-Match Prevention](002_self_match_prevention.md)); counting it
   would let the pre-check pass on liquidity step 3 cannot use.
3. **Match loop.** While the order has remainder and the best opposite
   order crosses (buy limit ≥ best ask; sell limit ≤ best bid; market
   orders always cross a non-empty side): fill against opposite orders in
   **price priority first, time priority within a price level** — better
   price always first; within one price, strictly oldest first, FIFO with no
   exceptions.
4. **Split exactly.** A partial fill splits quantity and settlement
   conservation-exact: the parts sum to the original with zero remainder
   slack, per [`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md)'s
   splitting contract. Each fill releases the matched portion of both
   sides' reservations in the same event.
5. **Dispose remainder by TIF.** **FOK**: cancel whole (per step 2's
   pre-check, reachable only if the book moved between check and match —
   still all-or-nothing); the design corpus's own matching system states
   the identical outcome directly — kill the order outright when it
   cannot fill in full immediately, no partial fills (→ external design
   corpus `ecs_system/049_order_matching_system.md`, plain-text
   citation). **IOC**: cancel the unfilled remainder immediately,
   releasing its reservation. **GTC**: rest the remainder on the book at
   its limit price, reservation retained. The corpus's own condition
   vocabulary names only Fill-or-Kill and Good-till-Cancelled
   (→ external design corpus
   `ecs_component/152_limit_order_conditions.md`, plain-text citation) —
   **IOC** is this instance's own addition, not one the source material
   names.
6. **Emit events.** Every state change above — acceptance, each fill, each
   release, rest, cancel — is one emitted event; the event stream is the
   recovery source (→ [Exchange Core v0.1](../feature/001_exchange_core_v0_1.md)
   exit criterion 4), so no step mutates state without its event.

**Closed at implementation (2026-08-30), with reasoning:**

- The **executed-price rule** is the **resting order's price** — the maker's.
  It is the only price both parties saw before the trade existed. Executing at
  the taker's limit instead transfers the whole spread to whichever side
  happened to arrive second, which is a fee charged by the venue that no fee
  schedule accounts for and no participant agreed to; and since a taker
  sweeping several levels would then pay one price for all of them, the two
  choices are not interchangeable at depth. Each trade therefore executes at
  its own maker's price, never at one blended price. The rule lives in
  `exchange_fill` as `Trade::executed_price`, taking both prices and
  returning one, so it has a single home rather than being an expression
  inlined in the match loop.
- **Book representation** is a **sorted `VecDeque` of price levels per
  side**, best at the front, each level a FIFO queue (`exchange_level`). The
  observable property this algorithm actually promises is the published
  order, and a test reads it directly. Insertion finds its position by
  `partition_point`. Matching only ever consumes the front, so an emptied
  level leaves in O(1): 40 000 one-order levels sweep in 0.9 ms, against
  452 ms when each side was a `Vec` and every emptied level shifted the rest.
  The change sat behind `insert`/`best`/`consume_best` and did not touch this
  algorithm.

Both choices are recorded rather than merely made, because both are reversible
and neither was forced by the corpus: the design corpus ties the engine to no
algorithm or data structure beyond a named lock-free pattern — no source gives
its matching algorithm or order-book data flow (→ external design corpus
`system/032_order_matching_engine.md`, plain-text citation).

**Still open at this grain — named, not decided here:**

- **Stop-order trigger semantics** — when a Stop becomes an executable
  Market/Limit order, and against which price feed. Untouched: the
  implementation is limit-only.
- **Fee rate and schedule** — flat, maker/taker pair, volume-tiered, or
  per-account. Open. The fee *hook position* is no longer open: fees are
  assessed at exactly one point, in step 3 at trade generation, against that
  fill's own notional, and nowhere in steps 1, 2, 4, or 5
  (→ [Fee Assessment Point](004_fee_assessment_point.md)). Nothing is charged
  while the schedule is open.
- **Time-in-Force disposition.** Step 5's `{ FOK, IOC, GTC }` branch is
  specified and implemented — `exchange_match::cross` consults the order's own
  `tif`, and `exchange_core::Exchange::exchange_step` threads a real,
  caller-chosen value through rather than pinning every order to `GTC`.

### Algorithms

| File | Relationship |
|------|--------------|
| [002_self_match_prevention.md](002_self_match_prevention.md) | Runs inside step 3 and supplies step 2's same-key exclusion from the FOK pre-check |
| [003_cancel_and_amend_semantics.md](003_cancel_and_amend_semantics.md) | The two operations that interrupt a resting order between steps, and the race this pipeline's positions decide |
| [004_fee_assessment_point.md](004_fee_assessment_point.md) | Closes the fee hook position at step 3; leaves the rate open |

### Features

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md) | Step 4's exact fill split is performed under this feature's conservation-exact splitting contract |
| [../feature/001_exchange_core_v0_1.md](../feature/001_exchange_core_v0_1.md) | Exit criterion 1 exercises this pipeline across the full order-type × TIF matrix |

### Invariants

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_add/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_add/readme.md) | Why step 4's split can promise zero remainder slack |
| [../invariant/001_escrow_covers_resting_orders.md](../invariant/001_escrow_covers_resting_orders.md) | Steps 2, 4, and 5 are where its reserve and release obligations execute |
| [../invariant/002_arrival_sequence_total_and_replayable.md](../invariant/002_arrival_sequence_total_and_replayable.md) | Supplies the total order step 3's "strictly oldest first" is a predicate over |
| [../invariant/003_available_and_reserved_balance_separation.md](../invariant/003_available_and_reserved_balance_separation.md) | The two-sided holding step 2 moves quantity across, and steps 4–5 move it back |

### Pitfalls

| File | Relationship |
|------|--------------|
| [../pitfall/001_balance_lost_update.md](../pitfall/001_balance_lost_update.md) | Steps 2, 4, and 5 are exactly the balance mutations this pitfall's read-modify-write trap sits on |

### Protocols

| File | Relationship |
|------|--------------|
| [../protocol/001_trade_event_stream.md](../protocol/001_trade_event_stream.md) | The five kinds step 6 emits, and what a consumer of them may rely on |

### State Machines

| File | Relationship |
|------|--------------|
| [../state_machine/001_order_lifecycle.md](../state_machine/001_order_lifecycle.md) | The states each step moves an order between, and the invariants no step may break |

### Sources

| File | Relationship |
|------|--------------|
| `ecs_component/152_limit_order_conditions.md` (design corpus) | The corpus's own Fill-or-Kill/Good-till-Cancelled vocabulary; IOC is this instance's own addition |
| `ecs_system/049_order_matching_system.md` (design corpus) | States step 5's FOK outcome directly: kill outright, no partial fills |
| `src/lib.rs` | `Exchange::exchange_step`/`step_place` — steps 1, 2, 5 and 6; step 5's TIF branch is implemented, threaded through from the caller's own order |
| `../../../exchange_match/src/lib.rs` | Step 3's match loop and step 4's exact split, at the maker's price |
| `../../../exchange_book/src/lib.rs` | The book representation this instance leaves to implementation, and its published order |
| `../../../exchange_escrow/src/lib.rs` | Step 2's reservation and the settlement that releases price improvement |
| `system/032_order_matching_engine.md` (design corpus) | Confirms matching algorithm and book representation are left unspecified by the corpus itself |

### Tests

| File | Relationship |
|------|--------------|
| `tests/matching_test.rs` (to create) | The order-type × TIF matrix; FIFO-within-level property; FOK all-or-nothing; exact split sums |
