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
| `ExchangeError` | enum | `Rejected(RejectReason) \| Escrow(EscrowError) \| Matching(MatchError) \| NotResting(OrderId) \| UnknownInstrument(InstrumentId) \| Spec(SpecError) \| SpecAlreadyRegistered(InstrumentId) \| Depth(DepthError) \| Halt(HaltError)` |
| `StepOutcome` | enum, `#[must_use]` | `Placed(Result<Receipt, ExchangeError>) \| Cancelled(Result<Quantity, ExchangeError>) \| ReplaceNotWired` — what one drained `InboundCmd` produced; `ReplaceNotWired` is a deliberate deferral, not an error (see "Differs from the proposal") |
| `PostingAsset` | enum | `Cash \| Asset` — the asset key `postings()` tags each entry with, so `verify` nets cash and asset separately; escrow's own two holdings, not an instrument's `AssetId`s |
| `EscrowPort` | trait, `: Clone` | `fn reserve(&mut self, &Order) -> Result<Obligation, EscrowError>; fn release(&mut self, AccountId, OrderId) -> Result<Obligation, EscrowError>; fn settle(&mut self, &Trade, Side, Price) -> Result<(), EscrowError>` — the facade's only route to `reserve`/`release`/`settle`; implemented for `Escrow` (see ADR-002) |
| `Exchange< E = Escrow >` | struct | private fields (`book`, `escrow`, `next_order`, `next_sequence`, `events`, `specs`, `caps`, `stats`); on `Exchange< Escrow >` only: `fn new() -> Self; fn open_account(&mut self, AccountId, Money, Quantity) -> Result<(), ExchangeError>`; on `Exchange< E : EscrowPort >`: `fn with_escrow(E) -> Self; fn spec_register(&mut self, InstrumentId, AssetId, AssetId, Price, Quantity) -> Result<(), ExchangeError>; fn caps_set(&mut self, InstrumentId, BookCaps); fn depth_get(&self, InstrumentId, usize) -> Result<Depth, ExchangeError>; fn halt_set(&mut self, InstrumentId) -> Result<(), ExchangeError>; fn halt_clear(&mut self, InstrumentId) -> Result<(), ExchangeError>; fn halt_is(&self, InstrumentId) -> Result<bool, ExchangeError>; fn snap_take(&self, InstrumentId, Money) -> BookSnap; fn event_drain(&mut self) -> Vec<Event>; fn stats_get(&self) -> BookStats; fn exchange_step(&mut self, &mut Consumer<'_, InboundCmd>, SelfMatchPolicy) -> Vec<StepOutcome>; fn cancel(&mut self, OrderId) -> Result<Quantity, ExchangeError>; fn events(&self) -> &[Event]; fn book(&self) -> &Book; fn escrow(&self) -> &E; fn postings(&self) -> Result<Vec<Entry<AccountId, PostingAsset>>, ExchangeError>` — `submit` is gone, replaced by `exchange_step`; `caps`/`caps_set` added when `exchange_cap` was wired into `step_place`'s dry run |

### Re-exported surface — declared elsewhere, re-exported through this facade

| Source Crate | Re-exported Items |
|------|------|
| `exact_arith` | `Backing, ConservationError, Entry, KindError, MONEY_SCALE, Money, Quantity, Report, verify` |
| `exchange_book` | `Book, Resting` |
| `exchange_cap` | `BookCaps` |
| `exchange_depth` | `Depth, DepthError, LevelView` |
| `exchange_escrow` | `Account, Conserved, Escrow, EscrowError, Holding` |
| `exchange_fill` | `CancelCause, Event, EventKind, RejectReason, Trade` |
| `exchange_halt` | `HaltError` |
| `exchange_id` | `AccountId, InstrumentId, OrderId` |
| `exchange_inbound` | `BuildError, Consumer, Drain, Ends, InboundCmd, Producer, RingConfig, Split, inbound_flush, inbound_overflow_reject, inbound_ring` |
| `exchange_match` | `Crossing, MatchError, SelfMatchCancellation, SelfMatchPolicy` |
| `exchange_order` | `Amount, Obligation, Order` |
| `exchange_seq` | `Sequence` |
| `exchange_side` | `Side` |
| `exchange_snap` | `BookSnap, RestRow` |
| `exchange_spec` | `AssetId, InstrumentSpec, SpecError` |
| `exchange_stats` | `BookStats` |
| `exchange_tif` | `Tif` — now caller-selectable (re-export restored once `exchange_step` made TIF a real per-order field instead of a hardcoded `Gtc`) |
| `exchange_types` | `Price, TypeError, notional, obligation` — narrowed from sixteen items to these four when its eleven-type re-export retired on 2026-10-05; the other twelve now come from their owning leaf crate's own row above (`AccountId`/`OrderId` moved to `exchange_id`, `Amount`/`Obligation`/`Order` to `exchange_order`, `Sequence` to `exchange_seq`, `Side` to `exchange_side`, `CancelCause`/`Event`/`EventKind`/`RejectReason`/`Trade` to `exchange_fill`) |

`exchange_seq::seq_next` and `exchange_rest::rest_place` are used internally
(a private `use`, not `pub use`) and are not part of the re-exported surface —
`exchange_step` owns sequencing and resting itself rather than exposing either
call directly; see `exchange_step`'s own doc comment.

### Differs from the proposal

The source design's own exposed-item list for crate 23
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:663-672`,
also catalogued centrally at
[`../../../../docs/exposed_item/023_exchange_core_items.md`](../../../../docs/exposed_item/023_exchange_core_items.md)
and [`../../../../docs/crate/023_exchange_core.md`](../../../../docs/crate/023_exchange_core.md))
proposes a facade over ten sub-crates: `Exchange { books, specs, inbound,
events, stats }`, `exchange_new`, `spec_register`,
`order_submit`/`order_cancel`/`order_replace`, a caller-driven `exchange_step`,
and `depth_get`/`halt_set`/`snap_take`/`event_drain`/`stats_get`.

As of the Stage 9 facade rework (2026-10-03), most of that gap is closed:
`spec_register`, `depth_get`, `halt_set`/`halt_clear`/`halt_is`, `snap_take`,
`event_drain`, `stats_get`, and a caller-driven `exchange_step` are all real,
over fourteen sub-crates now (every row in the table above) rather than four.
What still genuinely diverges:
- `exchange_new` vs. the real `Exchange::new` — a naming difference only.
- `order_submit`/`order_cancel` don't exist by those names — `exchange_step`
  applies `InboundCmd::Place`/`Cancel` drained from a ring instead of taking
  direct call arguments; `cancel` remains a direct, non-ring method,
  unchanged, and `exchange_step`'s own `Cancel` arm delegates to it rather
  than re-implementing it.
- `order_replace` has no facade-level equivalent yet: `exchange_step` drains
  `InboundCmd::Replace` but returns `StepOutcome::ReplaceNotWired` rather than
  applying it — a deliberate scope cut (no escrow-aware replace orchestration
  or `EventKind::Replaced` variant exists yet), not an oversight. See
  [`../decisions/001_submit_replaced_by_ring_fed_exchange_step.md`](../decisions/001_submit_replaced_by_ring_fed_exchange_step.md)
  for the full reasoning, and
  [`../../../exchange_rest/docs/pitfall/002_rest_replace_trusts_new_resting_instrument.md`](../../../exchange_rest/docs/pitfall/002_rest_replace_trusts_new_resting_instrument.md)
  for why this deferral also happens to make an unrelated `exchange_rest` gap
  unreachable through this facade, without fixing it.
- Market orders, amend, and fees remain unimplemented — named in this crate's
  own module doc, "What this slice still does not implement."

`open_account` and `postings` remain real items the proposal never named for
this crate. This is a real, already-disclosed divergence — don't re-litigate
it here; it is tracked centrally in the two files cited above.
