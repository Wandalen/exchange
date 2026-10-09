# Crate: exchange_level

### Scope

- **Purpose**: One price, FIFO rest.
- **Responsibility**: Hold every order resting at a single price, in arrival order.
- **In Scope**: One price only.
- **Out of Scope**: Any other price — that is the book's concern.

**Design status**: Built, in its own `exchange_level` crate — that summary predated this crate's extraction. Verified built-vs-proposed comparison: [`../../module/exchange_level/docs/item/readme.md`](../../module/exchange_level/docs/item/readme.md).
Also depends directly on `exact_arith` (006's facade; proposed leaves were
`exact_kind`, `exact_cmp`) — see
`../neighbor_contract/001_006_supplies_money_qty_price.md`'s per-crate table.

### Statement

Without a dedicated level, there is no price-time priority to speak of. This crate closes hard problems 2 (price-time) and 3 (partial fills), and feature 7 (price levels, FIFO inside a level). It depends on `exchange_order` and `exchange_seq`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:398-403` | Crate 8 in the source's Prompt 2 answer for workstream 002 |
