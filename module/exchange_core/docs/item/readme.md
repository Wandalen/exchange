# item

The exposed surface of `exchange_core`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice, consistent
with `exchange_id`'s/`exchange_escrow`'s own `docs/item/readme.md`).

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item this crate itself declares, plus a separate accounting of what it re-exports from the crates it faces — and a note on fidelity to the proposal.
- **In Scope**: This crate's own public surface, both declared here and re-exported through here.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above); the items themselves where they are actually declared (→ each source crate's own `docs/item/readme.md`).

### Overview Table — declared in this crate

| Item | Kind | Signature |
|------|------|-----------|
| `Receipt` | struct | `{ pub order: OrderId, pub trades: Vec<Trade>, pub resting: Quantity, pub self_match_cancelled: bool }`, plus `fn is_complete(&self) -> bool` |
| `ExchangeError` | enum | `Rejected(RejectReason) \| Escrow(EscrowError) \| Matching(MatchError) \| NotResting(OrderId)` |
| `Exchange` | struct | private fields (`book`, `escrow`, `next_order`, `next_sequence`, `events`); `fn new() -> Self; fn open_account(&mut self, AccountId, Money, Quantity) -> Result<(), ExchangeError>; fn submit(&mut self, AccountId, Side, Price, Quantity) -> Result<Receipt, ExchangeError>; fn cancel(&mut self, OrderId) -> Result<Quantity, ExchangeError>; fn events(&self) -> &[Event]; fn book(&self) -> &Book; fn escrow(&self) -> &Escrow; fn postings(&self) -> Result<Vec<Entry>, ExchangeError>` |

### Re-exported surface — declared elsewhere, re-exported through this facade

| Source Crate | Re-exported Items |
|------|------|
| `exact_arith` | `Backing, ConservationError, Entry, KindError, MONEY_SCALE, Money, Quantity, Report, verify` |
| `exchange_book` | `Book, Resting` |
| `exchange_escrow` | `Account, Conserved, Escrow, EscrowError, Holding` |
| `exchange_id` | `InstrumentId` |
| `exchange_match` | `Crossing, MatchError, SelfMatchCancellation, SelfMatchPolicy` |
| `exchange_types` | `AccountId, Amount, CancelCause, Event, EventKind, Obligation, Order, OrderId, Price, RejectReason, Sequence, Side, Trade, TypeError, notional, obligation` |

`exchange_seq::seq_next` and `exchange_tif::Tif` are used internally (a
private `use`, not `pub use`) and are not part of the re-exported surface —
`Tif` is not yet caller-selectable; see "Differs from the proposal" below.

### Differs from the proposal

The source design's own exposed-item list for crate 23
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:663-672`,
also catalogued centrally at
[`../../../../docs/exposed_item/023_exchange_core_items.md`](../../../../docs/exposed_item/023_exchange_core_items.md)
and [`../../../../docs/crate/023_exchange_core.md`](../../../../docs/crate/023_exchange_core.md))
proposes a wider facade: `Exchange { books, specs, inbound, events, stats }`,
`exchange_new`, `spec_register`, `order_submit`/`order_cancel`/`order_replace`,
a caller-driven `exchange_step`, and `depth_get`/`halt_set`/`snap_take`/
`event_drain`/`stats_get` — composing ten sub-crates.

The real build is narrower, over four sub-crates (`exchange_types`,
`exchange_book`, `exchange_match`, `exchange_escrow`) plus `exact_arith`
directly — `exchange_spec`, `exchange_depth`, `exchange_halt`, `exchange_snap`,
and `exchange_stats` do not exist yet, so this facade has no accessor for any
of them. `submit`'s five-step order (validate → reserve → match → settle →
rest) runs as one atomic internal call rather than a caller-driven
`exchange_step`. There is no `order_replace` — no atomic replace exists
anywhere in the family yet. `open_account` and `postings` (a bridge to
`exact_arith`'s conservation auditor) are real items the proposal never named
for this crate. This crate's own module doc (`src/lib.rs` lines 33–70)
self-discloses what is not yet wired: Time-in-Force (`Order` carries a real
`tif` field, but `submit` pins every order to `Tif::Gtc`), multi-instrument
routing (`Book` is keyed by instrument, but this facade pins every order to
one constant `InstrumentId`), market orders, amend, fees, and a
per-book-configurable self-match policy (`submit` hardcodes
`SelfMatchPolicy::CancelIncoming`).

This is a real, already-disclosed divergence (narrower surface than
proposed, not a hidden gap) — don't re-litigate it here; it is tracked
centrally in the two files cited above.
