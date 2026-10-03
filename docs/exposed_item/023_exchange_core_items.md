# Exposed Item: exchange_core

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_core`.
- **Responsibility**: The facade — the one crate that knows all the others exist.

**Design status**: Built as the real `exchange_core` crate (`module/exchange_core/src/lib.rs`), close in spirit but narrower in surface than proposed:
- Proposed: `Exchange { books, specs, inbound, events, stats }`, `exchange_new`, `spec_register`, `order_submit`/`order_cancel`/`order_replace`, `exchange_step`, `depth_get`/`halt_set`/`snap_take`/`event_drain`/`stats_get`, `ExchangeError`.
- Real: `Exchange { book, escrow, next_order, next_sequence, events }` (private fields; no `specs`/`inbound`/`stats` fields, since those crates don't exist), `Exchange::new`, `open_account`, `submit` (≈ `order_submit`, returns a `Receipt`), `cancel` (≈ `order_cancel`), `events()`/`book()`/`escrow()` accessors, `postings()` (a real item the proposal never named — a bridge to `exact_arith`'s conservation auditor), and `ExchangeError { Rejected, Escrow, Matching, NotResting }` — 4 variants, none matching the proposal's implied wrapper-of-children shape by name but serving the same wrapping role.
- No `spec_register`/`depth_get`/`halt_set`/`snap_take`/`stats_get` — each depends on a crate (`exchange_spec`, `exchange_depth`, `exchange_halt`, `exchange_snap`, `exchange_stats`) that does not exist.
- No `order_replace` — see `../exposed_item/015_exchange_rest_items.md`; no atomic replace exists anywhere in the family.
- No `exchange_step` as a caller-visible phased function — `submit`'s five-step order (validate → reserve → match → settle → rest) is internal to one call rather than something a caller drives explicitly (`exchange_core/src/lib.rs:8-25`).
- `open_account` is a real item the proposal's Prompt 3 answer for this crate never named (though workstream-010-style account creation is implied elsewhere in the source).
- The module's own doc comment (`exchange_core/src/lib.rs:33-61`) self-discloses five unimplemented pieces: Time-in-Force, market orders, amend, fees, and per-book-configurable self-match policy — the last one notable because it means even the one `SelfMatchPolicy` enum that *does* exist (`exchange_match`) is not yet reachable through this facade's own public API; `submit` hardcodes `SelfMatchPolicy::CancelIncoming`.

### Statement

Prompt 3 specifies a facade over ten sub-crates with five accessor functions and a step-by-step `exchange_step`. The real facade sits over only four sub-crates (`types`, `book`, `match`, `escrow`) plus `exact_arith` directly, runs its five-step submission order as one atomic internal call rather than a caller-driven step function, and has no accessor for depth, halt, snapshot, or stats — all unimplemented crates.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:663-672` | Crate `exchange_core`'s exposed-item list in the source's Prompt 3 answer, including its closing note on private/internal items and test placement |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
