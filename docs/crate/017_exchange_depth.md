# Crate: exchange_depth

### Scope

- **Purpose**: Top-N without walking every rest.
- **Responsibility**: Answer a depth query cheaply.
- **In Scope**: Read-only access to the book.
- **Out of Scope**: Any mutation — this crate only reads.

**Design status**: Built, in its own `exchange_depth` crate — matches the proposal exactly. Verified built-vs-proposed comparison: [`../../module/exchange_depth/docs/item/readme.md`](../../module/exchange_depth/docs/item/readme.md).
Also depends directly on `exact_arith` (006's facade; proposed leaf was
`exact_kind`) — see `../neighbor_contract/001_006_supplies_money_qty_price.md`'s
per-crate table.

### Statement

Without this crate, the UI would have to scan every rest just to answer a depth query. It closes hard problem 13 (depth) and feature 17 (`depth_top`). It depends on `exchange_book`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:452-457` | Crate 17 in the source's Prompt 2 answer for workstream 002 |
