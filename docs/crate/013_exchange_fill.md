# Crate: exchange_fill

### Scope

- **Purpose**: Fill, reject, cancel-ack.
- **Responsibility**: Give the match loop's three outcomes a typed shape.
- **In Scope**: The three output types.
- **Out of Scope**: Writing the wallet — this crate never does that.

**Design status**: Built, in its own `exchange_fill` crate (extracted out
of `exchange_types` this session) — `Trade`, `Event`, `EventKind` cover
this role (no type literally named `Fill`). Full writeup moved to the
crate's own docs: [`../../module/exchange_fill/readme.md`](../../module/exchange_fill/readme.md),
[`../../module/exchange_fill/docs/item/readme.md`](../../module/exchange_fill/docs/item/readme.md).
Also depends directly on `exact_arith` (006's facade; proposed leaf was
`exact_kind`) — see `../neighbor_contract/001_006_supplies_money_qty_price.md`'s
per-crate table.

### Statement

Without these types, workstream 010 has nothing to settle from. This crate closes hard problems 12 (events, not wallets) and 21 (maker and taker on the fill), and features 12 (`Fill`), 13 (`Reject`), and 14 (`CancelAck`). It depends on `exchange_id` and `exchange_order`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:428-433` | Crate 13 in the source's Prompt 2 answer for workstream 002 |
