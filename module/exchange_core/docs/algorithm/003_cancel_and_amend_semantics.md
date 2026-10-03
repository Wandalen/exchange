# Algorithm: Cancel and Amend Semantics

### Scope

- **Purpose**: Fix the two operations that touch an order after acceptance, since both race the match loop and one of them — amend — decides whether queue position can be bought rather than waited for.
- **Responsibility**: State cancel's totality, idempotence, and declared race winner; state which amendments preserve time priority and which forfeit it, with the reason; and require the escrow adjustment to be atomic with the amend.
- **In Scope**: What a cancel and an amend guarantee, what they report when they lose the race, and what each does to priority and to escrow.
- **Out of Scope**: The pipeline these interrupt (→ [Price-Time Priority Matching](001_price_time_priority_matching.md)); the total order that decides both races (→ [Arrival Sequence Is Total and Replayable](../invariant/002_arrival_sequence_total_and_replayable.md)); session-level mass-cancel and cancel-on-disconnect, which decompose into individual cancels in that same order and change nothing here (→ [Trade Event Stream](../protocol/001_trade_event_stream.md)'s request cause).

### Abstract

Cancel and amend are the only operations that touch an order between
acceptance and a terminal state, and both are races against a match loop that
may fill the order in the same instant. The cancel half follows a familiar
shape for this kind of race: totality, and a serialized race with a
*reported* winner rather than a silent one, decided by position rather than
wall-clock timing. Where such a race's serialization point could in principle
be left open, here it is decided, because `task/decisions.md` Q-07 puts every
operation through one merged arrival order.

The amend half has no precedent anywhere in the source material — no amend,
modify, or cancel-replace operation exists in the design corpus at all, and
neither does any statement about its effect on priority. It is originated
here, and its central question is not mechanical.

### Algorithm

**Cancel.**

1. **A cancel takes a position in the same merged arrival order as an order
   submission** (→ [Arrival Sequence Is Total and Replayable](../invariant/002_arrival_sequence_total_and_replayable.md)).
   It therefore races nothing in the wall-clock sense: the race against a fill
   is decided by position, and the position is claimed before either operation
   is visible. This is what makes the outcome reproducible under replay rather
   than a function of scheduling.

2. **Exactly one of two outcomes, and which one occurred is reported.**
   Either the cancel precedes the fill — the remainder is cancelled, its
   reservation released in the `OrderCancelled` event, and no `Trade` against
   that remainder exists anywhere in the stream — or the fill precedes the
   cancel, in which case the cancel applies to whatever remainder survives and
   reports the filled quantity it arrived too late for. Both are answers.
   Neither is a silent loss, and neither depends on timing the caller cannot
   observe. A caller that overrode something on the strength of a cancel needs
   to know whether the pre-override fill already happened, because the two
   cases demand different repair.

3. **Cancel is idempotent, and a repeat is a reportable non-error.** A second
   cancel of the same order returns *already terminal* and changes nothing —
   never a panic, and above all never a second release. Releasing a second
   time credits units nobody debited, which is a mint
   (→ [Available and Reserved Balance Separation](../invariant/003_available_and_reserved_balance_separation.md));
   idempotence here is a conservation requirement, not an ergonomic one.

4. **Cancel is total in the mathematical sense too** — defined for every order
   id, including one that never existed. Losing the race is routine, so a
   mechanism that treats an unrecognised id as a caller error puts a crash on
   the caller's ordinary path at exactly the frequency the race occurs
   (→ [Order Lifecycle](../state_machine/001_order_lifecycle.md) BI1).
   **Open:** whether the engine retains terminal orders long enough to
   distinguish *never existed* from *already terminal and evicted*. Both are
   non-error outcomes either way; the retention window decides only how
   informative the answer is, and it trades memory against diagnosability.

**Amend.**

5. **An amend applies in full or not at all.** No other participant ever
   observes an intermediate state in which the order is off the book, holds
   the wrong reservation, or exists twice. In particular an amend is *not* a
   cancel followed by a place: that decomposition has an instant in which the
   order is absent from the book and its escrow released, and both halves of
   that instant are exploitable (→ step 7).

6. **The priority rule — decided:**

   | Amendment | Time priority | Resulting position |
   |-----------|---------------|--------------------|
   | Quantity **decrease** | **Preserved** | Arrival position unchanged |
   | Quantity **increase** | **Forfeited** | New position, back of the same price level |
   | Price **change**, either direction | **Forfeited** | New position, back of the destination level |
   | Side or order-type change | **Not an amend** | Rejected; the operation for this is cancel-then-place |

   **Why an increase must forfeit.** A queue position is a claim on being
   filled before orders that arrived later. Quantity that was never queued
   holds no such claim. If an increase preserved position, an account could
   rest one unit early, watch the level build behind it, and then amend to ten
   thousand units — acquiring priority ahead of everyone who actually waited,
   for the price of one unit's exposure. That is not a subtle edge case: it
   makes queue position purchasable at the cost of the smallest legal order,
   and it reduces
   [Price-Time Priority Matching](001_price_time_priority_matching.md) step 3's
   "strictly oldest first, FIFO with no exceptions" to decoration.

   **Why a decrease may preserve.** The mirror argument: a decrease releases
   claim and never acquires it, and the units that remain are the same units
   that queued. Nothing ranks ahead of an order after a decrease that did not
   rank ahead of it before, so no other participant is worse off. This is the
   same rule a partial fill already follows without being called an amend
   (→ [Order Lifecycle](../state_machine/001_order_lifecycle.md) BI3).

   **Why a price change forfeits, including an improving one.** Priority is
   defined *within* a price level, so a move between levels has no position to
   preserve. The tempting exception — that improving the price (raising a bid,
   lowering an ask) deserves to keep something — fails on who is owed:
   priority at the destination level belongs to the orders already resting
   there, and they did not have to improve to earn it.

7. **Escrow is adjusted in the same event as the amend, never in two steps.**
   An amend that *increases* the obligation reserves the difference and is
   **rejected whole** if the difference cannot be reserved — the order remains
   exactly as it was, with its original quantity, price, and priority, because
   a partially-applied amend that shrank the order but failed to reserve is
   unrepresentable. An amend that *decreases* the obligation releases the
   difference inside the amend event. A release-then-reserve decomposition is
   what makes this a rule rather than an implementation note: between the two
   steps the account's `available` is higher than any correct state, and that
   is precisely the window in which a second order could be accepted against
   units already committed — the over-commitment
   [Available and Reserved Balance Separation](../invariant/003_available_and_reserved_balance_separation.md)
   exists to make impossible.

8. **Emit `OrderAmended`, carrying whether priority was retained**
   (→ [Trade Event Stream](../protocol/001_trade_event_stream.md)). The
   outcome is recorded rather than re-derived, so a consumer reading an
   archived stream never has to know which version of the rule above was in
   force when the amend was applied.

**Open — whether amend exists in v0.1 at all.**
[Exchange Core v0.1](../feature/001_exchange_core_v0_1.md) does not list it
among its commitments, and the design corpus contains no amend operation of
any kind. This instance specifies the semantics so that adding the operation
later is an increment rather than a redesign — and specifically so that the
priority rule is decided before an implementation decides it by accident,
which is the failure mode a rule discovered during coding always has. The
version slot is not this instance's to assign.

### Algorithms

| File | Relationship |
|------|--------------|
| [001_price_time_priority_matching.md](001_price_time_priority_matching.md) | The FIFO clause an amending quantity increase would defeat if it preserved position; the fill this cancel races |
| [002_self_match_prevention.md](002_self_match_prevention.md) | The other producer of `OrderCancelled` events, distinguished from a requested cancel by cause |
| [004_fee_assessment_point.md](004_fee_assessment_point.md) | Why neither operation here assesses anything — a fee on cancel would price the withdrawal of liquidity |

### Features

| File | Relationship |
|------|--------------|
| [../feature/001_exchange_core_v0_1.md](../feature/001_exchange_core_v0_1.md) | Does not yet list amend among its commitments — the open version-slot question above |

### Invariants

| File | Relationship |
|------|--------------|
| [../invariant/001_escrow_covers_resting_orders.md](../invariant/001_escrow_covers_resting_orders.md) | Why an amend's reservation change rides the event, and why a rejected increase must leave the order untouched |
| [../invariant/002_arrival_sequence_total_and_replayable.md](../invariant/002_arrival_sequence_total_and_replayable.md) | Supplies the position that decides both races, and makes the outcome reproducible under replay |
| [../invariant/003_available_and_reserved_balance_separation.md](../invariant/003_available_and_reserved_balance_separation.md) | The window a release-then-reserve amend would open, and the mint a double release would produce |

### Protocols

| File | Relationship |
|------|--------------|
| [../protocol/001_trade_event_stream.md](../protocol/001_trade_event_stream.md) | Carries the amend's retained-or-forfeited outcome as a stored field, and the cancel's cause |

### State Machines

| File | Relationship |
|------|--------------|
| [../state_machine/001_order_lifecycle.md](../state_machine/001_order_lifecycle.md) | BI1 — losing either race is a reportable non-error; the amend self-transitions this instance justifies |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs` | `Exchange::cancel` — release and removal in one operation; amend is not implemented |
| `../../../exchange_book/src/lib.rs` | `Book::cancel` — returns `None` for an absent order rather than panicking |

### Tests

| File | Relationship |
|------|--------------|
| `tests/cancel_amend_test.rs` (to create) | A cancel interleaved with the fill that would consume its order yields exactly one of the two declared outcomes, and the reported outcome matches the event stream; a repeated cancel releases nothing further; a quantity decrease preserves position and an increase does not, checked by the fill order of a rebuilt level; an amend whose increase cannot be reserved leaves quantity, price, position, and reserved amount all unchanged |
