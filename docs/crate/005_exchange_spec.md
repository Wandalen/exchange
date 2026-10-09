# Crate: exchange_spec

### Scope

- **Purpose**: Tick, lot, the asset pair, and halt on the instrument.
- **Responsibility**: Carry an instrument's own grid and halt flag.
- **In Scope**: Snap only — the decimal implementation itself lives in 006.
- **Out of Scope**: Decimal arithmetic.

**Design status**: Built, matching this proposal exactly. Full writeup moved
to the crate's own docs: [`../../module/exchange_spec/readme.md`](../../module/exchange_spec/readme.md),
[`../../module/exchange_spec/docs/item/readme.md`](../../module/exchange_spec/docs/item/readme.md).
Also depends directly on `exact_arith` (006's facade; proposed leaves were
`exact_kind`, `exact_snap`) — see
`../neighbor_contract/001_006_supplies_money_qty_price.md`'s per-crate table.

### Statement

Without an instrument spec, an illegal price can enter the book with no grid to reject it against. This crate closes hard problems 1 (one book per instrument), 8 (tick and lot), 18 (halt), and 19 (more than one asset), and features 4 (`InstrumentSpec`) and 5 (`price_snap`, `qty_snap`). It depends on `exchange_id` and workstream 006's types.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:380-385` | Crate 5 in the source's Prompt 2 answer for workstream 002 |
