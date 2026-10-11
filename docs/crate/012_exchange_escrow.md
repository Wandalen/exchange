# Crate: exchange_escrow

### Scope

- **Purpose**: A hold/release/commit port.
- **Responsibility**: Give the match loop a way to lock funds before it rests an order.
- **In Scope**: The port itself. No balances — workstream 010 implements the port.
- **Out of Scope**: Balances.

**Design status**: Built, scope-expanded: `Escrow` holds real `Account`
balances (`open`, `total_cash`, `total_asset`) rather than leaving them to
workstream 010, and names its methods `reserve`/`release`/`settle`. The port
is `exchange_core::EscrowPort`, which `Escrow` implements. Built-vs-proposed
comparison:
[`../../module/exchange_escrow/docs/item/readme.md`](../../module/exchange_escrow/docs/item/readme.md).
Also depends directly on `exact_arith` (006's facade; proposed leaf was
`exact_kind`) — see `../neighbor_contract/001_006_supplies_money_qty_price.md`'s
per-crate table.

### Statement

Without this crate, a match could spend funds it never locked. It closes hard problem 4 (escrow before rest) and feature 11 (`EscrowPort`). It depends on `exchange_id` and `exchange_order`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:422-427` | Crate 12 in the source's Prompt 2 answer for workstream 002 |
