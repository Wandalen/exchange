# Crate: exchange_conserve

### Scope

- **Purpose**: A set of fills sums to zero.
- **Responsibility**: Assert conservation over a batch of fills.
- **In Scope**: The assertion itself.
- **Out of Scope**: Walking the book — this crate does not do that.

**Design status**: Built — `conserve_assert`, called by
`exchange_match::cross` on every batch. `fill_legs_sum` is not built:
`exact_arith::money_sum_assert_zero` does the sum. `ConserveError` has three
variants. Built-vs-proposed comparison:
[`../../module/exchange_conserve/docs/item/readme.md`](../../module/exchange_conserve/docs/item/readme.md).
"Workstream 006's types" below resolves to `exact_arith` directly (006's
facade; proposed leaves were `exact_conserve`, `exact_kind`) — see
`../neighbor_contract/001_006_supplies_money_qty_price.md`'s per-crate table.

### Statement

Without this check, the machine can silently print money. This crate closes hard problem 5 (conservation) and feature 21 (`conservation_assert`). It depends on `exchange_fill` and workstream 006's types.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:434-439` | Crate 14 in the source's Prompt 2 answer for workstream 002 |
