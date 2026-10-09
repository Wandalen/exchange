# Crate: exchange_snap

### Scope

- **Purpose**: Plain rows of the rests, and the tick, from the caller.
- **Responsibility**: Give the book a copyable snapshot shape.
- **In Scope**: The row type itself. Not file format 012.
- **Out of Scope**: Any particular save-file format.

**Design status**: Built, in its own `exchange_snap` crate — `RestRow`/`BookSnap` match the proposal's fields exactly; the two real divergences are a dropped `exchange_order` dependency and no `SnapError`. Verified built-vs-proposed comparison: [`../../module/exchange_snap/docs/item/readme.md`](../../module/exchange_snap/docs/item/readme.md).
Also depends directly on `exact_arith` (006's facade; proposed leaf was
`exact_kind`) — see `../neighbor_contract/001_006_supplies_money_qty_price.md`'s
per-crate table.

### Statement

Without a snapshot, the market dies the moment the process saves and restarts. This crate closes hard problems 16 (closed types) and 22 (snapshot of a book), and feature 27 (book snapshot rows). It depends on `exchange_order` and `exchange_book`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:470-475` | Crate 20 in the source's Prompt 2 answer for workstream 002 |
