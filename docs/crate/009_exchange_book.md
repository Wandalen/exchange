# Crate: exchange_book

### Scope

- **Purpose**: Sorted bid and ask ladders for one instrument.
- **Responsibility**: Hold the resting orders of one instrument in price order.
- **In Scope**: Storage and lookup. No match, no escrow, no `ring_*` import.
- **Out of Scope**: Matching and escrow.

**Design status**: Built — `Book`, `Resting`. One `Book` holds every
instrument, keyed by `InstrumentId`, rather than one per instrument. Depends
on `exchange_id`, `exchange_level` and `exchange_side`, not `exchange_spec`;
depth is `exchange_depth`'s and capacity `exchange_cap`'s, read through
`Book`'s own counts. Built-vs-proposed comparison:
[`../../module/exchange_book/docs/item/readme.md`](../../module/exchange_book/docs/item/readme.md).
Also depends directly on `exact_arith` (006's facade; proposed leaf was
`exact_cmp`) — see `../neighbor_contract/001_006_supplies_money_qty_price.md`'s
per-crate table.

### Statement

Without this crate there is no market at all. It closes hard problems 1 (one book per instrument), 13 (depth), 14 (hot path), and 17 (capacity), and features 6 (book) and 22 (sorted price walk). It depends on `exchange_level`, `exchange_spec`, and `exchange_id`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:404-409` | Crate 9 in the source's Prompt 2 answer for workstream 002 |
