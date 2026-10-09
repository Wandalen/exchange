# Crate: exchange_order

### Scope

- **Purpose**: One record for a resting order or a taker.
- **Responsibility**: Carry everything one order needs as a single typed record.
- **In Scope**: The order record itself.
- **Out of Scope**: Price levels — this crate has no concept of one.

**Design status**: Built, in its own `exchange_order` crate (not folded into `exchange_types` — that summary predates this crate's extraction). Verified built-vs-proposed comparison: [`../../module/exchange_order/docs/item/readme.md`](../../module/exchange_order/docs/item/readme.md).
Also depends directly on `exact_arith` (006's facade; proposed leaf was
`exact_kind`) — see `../neighbor_contract/001_006_supplies_money_qty_price.md`'s
per-crate table.

### Statement

Without this crate, an order degrades into an untyped tuple. It closes hard problems 16 (closed types) and 22 (snapshot of a book), and features 3 (`Order`) and 28 (sequence, no wall clock). It depends on `exchange_id`, `exchange_side`, and `exchange_tif`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:386-391` | Crate 6 in the source's Prompt 2 answer for workstream 002 |
