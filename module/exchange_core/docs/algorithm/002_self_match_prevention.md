# Algorithm: Self-Match Prevention

### Scope

- **Purpose**: Decide what the engine *does* when an incoming order would cross a resting order from the same account, since the source material states the prohibition and supplies no policy — and a prohibition with no action is not implementable.
- **Responsibility**: Fix where detection happens, what "same account" is compared against, the three candidate policies with their information leakage and denial-of-service surfaces, and which parts are configurable versus fixed.
- **In Scope**: Detection point, identity comparison, policy set, the resumption rule, and the escrow and fee consequence of allowing a self-trade to execute.
- **Out of Scope**: The match loop this runs inside (→ [Price-Time Priority Matching](001_price_time_priority_matching.md)); the fee that makes an executed self-trade a value leak (→ [Fee Assessment Point](004_fee_assessment_point.md)); who authenticates a submitted identity, which no matching engine can (→ [Trade Event Stream](../protocol/001_trade_event_stream.md), and the host beyond it).

### Abstract

A self-cross is the one case where the match loop finds a crossing pair and
must not trade. The source material is unambiguous that it must not — the
engine is to be forbidden from matching two opposing orders of one and the
same participant (→ external design corpus
`conversation/anatolii_shlyakhto/0024_kostiantyn_required_topics_list.md`,
plain-text citation) — and equally unambiguous by omission about what happens
instead: no resolution policy, no identity scope, and no statement of the
effect on the passed-over resting order appear anywhere in it. Everything
below the prohibition is originated here.

The choice matters more than it looks. Each policy cancels a different order,
and cancelling an order is a public act: the book changes, and anybody
watching the book learns something. So the three policies differ not in
correctness but in **what they leak** and **what they let an attacker do** —
and one tempting fourth option leaks nothing and breaks price priority
instead.

### Algorithm

1. **Detect at crossing time, inside the match loop.** The check runs at
   [Price-Time Priority Matching](001_price_time_priority_matching.md)
   step 3, at the moment a specific incoming/resting pair is about to trade —
   never at acceptance. Acceptance-time detection would have to scan the book
   for the account's resting orders, cost order-count work on the hot path,
   and still be wrong: which pair actually crosses depends on what else fills
   first in the same pass, which is not known until the pass runs.

2. **Compare an opaque self-match key, defaulting to the account id.** "Same
   account" is the minimum, and it is not always the answer — several
   accounts under one controller are the same participant for this purpose,
   and the engine cannot discover that relationship. **Decided:** every order
   carries a self-match key; the engine compares keys for equality and never
   interprets one. That keeps a policy question outside a general-purpose
   engine, and keeps the comparison a constant-time equality rather than a
   lookup. **Open:** whether keys may be hierarchical, so that one parent key
   collides with several child keys. Flat equality is what is specified;
   hierarchy is an extension nothing here forecloses.

   **The engine trusts the key it is given.** Authenticating that a submitter
   is entitled to the key it claims is a session concern this crate has no
   means to perform, and it is load-bearing: under cancel-both (below), an
   attacker who can submit under a victim's key deletes the victim's orders.
   Stated here so the obligation lands somewhere rather than nowhere.

3. **Apply the configured policy** and emit the resulting cancellation(s)
   with cause `self-match`
   (→ [Trade Event Stream](../protocol/001_trade_event_stream.md)), so a
   consumer can distinguish a self-match cancellation from a requested one.

4. **Resume the match loop at the next resting order**, if the policy left
   the incoming order alive. The passed-over resting order lost no priority
   relative to anything else — it was *cancelled*, not skipped, so no order
   ranks ahead of it that did not before.

**The three policies:**

| Policy | Cancelled | What a third party learns | Denial-of-service surface | Cost in the honest case |
|--------|-----------|---------------------------|---------------------------|-------------------------|
| **Cancel-resting** | The resting order; the incoming order continues matching | A resting order vanishes at the instant an aggressor crossed it — a strong common-ownership signal linking two visible participants | Low externally; high self-inflicted — an account running a passive quote and an automated aggressive strategy destroys its own liquidity every time they meet | Loses the passive order, which is usually the one with queue position and therefore the valuable one |
| **Cancel-incoming** | The incoming order's remainder; the resting order survives | Least — the book does not change, so an observer sees only an order that arrived and did not trade, which is indistinguishable from many benign cases | An account's own resting order blocks its own access to every price level beyond it, so a stale quote silently degrades its owner's reach | Loses the aggressive order, which can be resubmitted; queue position is preserved |
| **Cancel-both** | Both remainders | Most — two simultaneous disappearances at one price level are a near-unambiguous common-ownership signal | Worst — one small resting order can be used to destroy a large incoming order's entire remainder, and the attack is available to anyone who can submit under the victim's key | Loses both sides, including the resting order's queue position |

**The tempting fourth option, named and rejected: skip-and-continue** — trade
through to the next resting order and leave the self-crossing one alone. It
cancels nothing, so it leaks nothing by disappearance, and that is exactly
why it is attractive. It fails on two counts. It leaves an order permanently
displayed at a price one participant cannot trade against, so the book is a
lie about executability for that participant — which is the property
[Escrow Covers Resting Orders](../invariant/001_escrow_covers_resting_orders.md)
exists to protect. And it fills the aggressor at a *worse* price than the
best displayed level, which is indistinguishable from a price-priority bug in
every log a support desk will ever look at.

**Fixed versus configurable:**

- **Fixed: a self-cross never trades.** Not a mode, not a flag, not a
  per-venue relaxation. The escrow and fee consequence below is why there is
  no "allow" setting to add later.
- **Configurable: which of the three policies applies, per book.** Per book —
  never per order. Letting a submitter choose which side dies hands the
  attacker in the cancel-both row a selector, which is strictly worse than
  the worst fixed configuration.
- **Open: the default.** Cancel-incoming is the conservative candidate — it
  never destroys resting liquidity and never changes the book — but nothing
  in the source material or in this repository's requirements selects one,
  and inventing a justification for a default is how a policy becomes
  permanent by accident.

**Decided — the interaction with all-or-nothing disposition, which is a
correction to an existing step rather than a preference.** An FOK order whose
complete fill *requires* crossing its own resting quantity cannot fill at
all, so it is cancelled whole rather than partially disposed. That means
[Price-Time Priority Matching](001_price_time_priority_matching.md) step 2's
pre-check must **exclude same-key resting quantity** from the "visible
opposite side" it measures against — otherwise the pre-check passes, step 3
cannot deliver, and the all-or-nothing promise breaks through an interaction
neither step anticipated on its own. That exclusion is now stated in step 2
itself. A partially-filled **IOC** that then self-crosses follows the
ordinary remainder rule — the remainder is cancelled, nothing is unwound —
and is not open either.

**The escrow and fee consequence — why the prohibition is structural.**

A self-trade that is allowed to execute moves units from an account to
itself. Both reservations are debited and both accounts credited, so
`available + reserved` is conserved per account
(→ [Available and Reserved Balance Separation](../invariant/003_available_and_reserved_balance_separation.md))
and the conservation audit balances
(→ [Conservation Audit](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md)).
Nothing is created and nothing is destroyed. And nothing was exchanged: the
only net movement is the fee, assessed against the traded notional
(→ [Fee Assessment Point](004_fee_assessment_point.md)), from the account to
the venue. That is a **pure value leak with a balanced ledger** — the audit's
entire detection capability is blind to it, because the fee is a real
transfer and real transfers are what the audit is designed to accept.

Three consequences follow, and they are the reason this is not merely
untidy:

- **The venue is paid for it.** A self-trade is fee revenue for a trade that
  transferred nothing, so the engine's operator has an incentive to leave the
  hole open and no automated check will ever surface it.
- **The printed volume is fictitious.** Every consumer downstream of the
  event stream (→ [Trade Event Stream](../protocol/001_trade_event_stream.md))
  treats a `Trade` as evidence that ownership moved. A self-trade makes that
  inference false while remaining a perfectly well-formed event, so the
  corruption propagates into anything computing volume, price history, or
  liquidity from the stream.
- **It prices market manipulation.** Without the ban, a participant can print
  arbitrary volume at arbitrary prices for the cost of the fee alone — which
  makes the fee schedule a manipulation-cost parameter, a role it was never
  designed for and cannot be tuned for without breaking its actual purpose.

### Algorithms

| File | Relationship |
|------|--------------|
| [https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md](https://github.com/Wandalen/exact/blob/master/module/exact_conserve/readme.md) | Balances across an executed self-trade — the detector that structurally cannot catch this |
| [001_price_time_priority_matching.md](001_price_time_priority_matching.md) | Hosts the detection point at step 3, and carries the FOK pre-check this instance corrects at step 2 |
| [004_fee_assessment_point.md](004_fee_assessment_point.md) | Supplies the fee that makes an executed self-trade a leak rather than a no-op |
| [003_cancel_and_amend_semantics.md](003_cancel_and_amend_semantics.md) | The other producer of `OrderCancelled` events; cancel/amend's own cancellations are distinguished from this policy's by cause |

### Features

| File | Relationship |
|------|--------------|
| [../feature/001_exchange_core_v0_1.md](../feature/001_exchange_core_v0_1.md) | Lists wash-trading safeguards in scope; this instance is the first of them with committed semantics |

### Invariants

| File | Relationship |
|------|--------------|
| [../invariant/001_escrow_covers_resting_orders.md](../invariant/001_escrow_covers_resting_orders.md) | The executability promise skip-and-continue would break by leaving an untradeable order displayed |
| [../invariant/003_available_and_reserved_balance_separation.md](../invariant/003_available_and_reserved_balance_separation.md) | Conserved across an executed self-trade, which is why the partition offers no detection either |

### Protocols

| File | Relationship |
|------|--------------|
| [../protocol/001_trade_event_stream.md](../protocol/001_trade_event_stream.md) | Carries the self-match key on acceptance and the self-match cause on cancellation, so a consumer distinguishes it from a requested cancel |

### State Machines

| File | Relationship |
|------|--------------|
| [../state_machine/001_order_lifecycle.md](../state_machine/001_order_lifecycle.md) | The Accepted → Cancelled and PartiallyFilled → Cancelled edges this policy triggers |

### Sources

| File | Relationship |
|------|--------------|
| `conversation/anatolii_shlyakhto/0024_kostiantyn_required_topics_list.md` (design corpus) | States the prohibition and no policy — the omission this instance fills |
| `../../../exchange_match/src/lib.rs` | Implements the key comparison at step 3 of the match loop, `SelfMatchPolicy`, and `Crossing.cancelled` |
| `../../src/lib.rs` | `Exchange::submit` selects the policy — currently hardcoded to `CancelIncoming`, the conservative candidate this instance's own "Open: the default" note above names — releases escrow for each cancellation, and emits it with `CancelCause::SelfMatch` |

### Tests

| File | Relationship |
|------|--------------|
| `tests/self_match_test.rs` | T09–T12: no `Trade` carries equal self-match keys on both sides across a mixed-account flow; each of the three configured policies cancels exactly its documented side and emits the self-match cause. T13 (an FOK whose only crossing liquidity carries its own key, cancelled whole) is not yet covered — no Time-in-Force exists on `Order` yet to construct an FOK from (→ `substrate/exchange/exchange_core/task/unverified/082_implement_exchange_core.md` History) |
