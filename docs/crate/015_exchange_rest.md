# Crate: exchange_rest

### Scope

- **Purpose**: Rest, cancel, replace.
- **Responsibility**: Own the three ways an order enters or leaves the book outside of matching.
- **In Scope**: Rest, cancel, replace.
- **Out of Scope**: Crossing — this crate does not cross.

**Design status**: Built as its own real crate — `rest_place`, `rest_cancel`,
`rest_replace`, and `RestReplaceError` all present, `rest_replace`'s atomic
cancel-then-reinsert-with-rollback the one genuinely new operation
(verified via direct read of `module/exchange_rest/src/lib.rs`). Not yet
wired into `exchange_core`'s own `Exchange::cancel`, but `exchange_inbound`'s
`inbound_apply` already calls through to all three functions — see
[`../../module/exchange_rest/readme.md`](../../module/exchange_rest/readme.md).
Carries no `exact_*` edge in `[dependencies]` either — `exact_arith` appears
only as a `[dev-dependencies]` entry (test fixtures) — confirming a
2026-10-07 user-supplied specification's claim that this crate has no direct
006 dependency; see `../neighbor_contract/001_006_supplies_money_qty_price.md`.

### Statement

Without this crate, there is no way into the book except through a match. It closes hard problems 9 (cancel and replace) and 15 (idempotent order ids), and features 8 (`book_rest`, `book_cancel`) and 25 (`book_replace`). It depends on `exchange_book`, `exchange_idem`, `exchange_cap`, `exchange_escrow`, and `exchange_spec`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:440-445` | Crate 15 in the source's Prompt 2 answer for workstream 002 |
