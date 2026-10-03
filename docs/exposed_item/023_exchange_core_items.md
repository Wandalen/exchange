# Exposed Item: exchange_core

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_core`.
- **Responsibility**: The facade — the one crate that knows all the others exist.

**Design status**: Built as the real `exchange_core` crate (`module/exchange_core/src/lib.rs`). As of the Stage 9 facade rework (2026-10-03), the gap against the proposal is mostly closed:
- Proposed: `Exchange { books, specs, inbound, events, stats }`, `exchange_new`, `spec_register`, `order_submit`/`order_cancel`/`order_replace`, `exchange_step`, `depth_get`/`halt_set`/`snap_take`/`event_drain`/`stats_get`, `ExchangeError`.
- Real: `Exchange { book, escrow, next_order, next_sequence, events, specs, stats }` (private fields), `Exchange::new`, `open_account`, `spec_register`, `depth_get`, `halt_set`/`halt_clear`/`halt_is`, `snap_take`, `event_drain`, `stats_get`, `exchange_step` (ring-fed, replaces the old `submit`), `cancel` (unchanged, called directly by `exchange_step`'s own `Cancel` arm), `events()`/`book()`/`escrow()` accessors, `postings()` (a real item the proposal never named — a bridge to `exact_arith`'s conservation auditor), `StepOutcome` (a real item the proposal never named — what one drained `InboundCmd` produced), and `ExchangeError { Rejected, Escrow, Matching, NotResting, UnknownInstrument, Spec, SpecAlreadyRegistered, Depth, Halt }` — 9 variants now, 5 added alongside the new accessors.
- `spec_register`/`depth_get`/`halt_set`/`snap_take`/`stats_get` are all real now — `exchange_spec`, `exchange_depth`, `exchange_halt`, `exchange_snap`, `exchange_stats` all landed as their own crates; `halt_clear`/`halt_is` are real too, beyond what the proposal's own `halt_set` name implied.
- `order_replace` still has no facade-level equivalent: `exchange_step` drains `InboundCmd::Replace` but returns `StepOutcome::ReplaceNotWired` rather than applying it — a deliberate scope cut (no escrow-aware replace orchestration or `EventKind::Replaced` variant exists yet), not a missing crate. See `module/exchange_core/docs/decisions/001_submit_replaced_by_ring_fed_exchange_step.md` for the full reasoning, and `../../module/exchange_rest/docs/pitfall/002_rest_replace_trusts_new_resting_instrument.md` for an unrelated `exchange_rest` gap this deferral happens to make unreachable through the facade, without fixing it.
- `exchange_step` is now a real, caller-driven, ring-fed function — `submit` is gone entirely, not merely wrapped.
- `open_account` remains a real item the proposal's Prompt 3 answer for this crate never named.
- The module's own doc comment (`exchange_core/src/lib.rs`, "What this slice still does not implement") discloses what remains genuinely unimplemented: market orders, amend, fees, and concurrent intake past the ring. Time-in-Force, multi-instrument routing, and per-call self-match policy are all real now, closed as a direct consequence of routing through `exchange_step`.

### Statement

Prompt 3 specifies a facade over ten sub-crates with five accessor functions and a step-by-step `exchange_step`. The real facade now sits over thirteen sub-crates (every re-export source in `module/exchange_core/docs/item/readme.md`) plus `exact_arith` directly, and has a real, caller-driven `exchange_step` plus all five named accessors. The one concrete gap remaining against the proposal is `order_replace` — deferred by deliberate decision, not a missing crate (see the Design status note above).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:663-672` | Crate `exchange_core`'s exposed-item list in the source's Prompt 3 answer, including its closing note on private/internal items and test placement |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
