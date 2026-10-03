# Crate: exchange_conserve

### Scope

- **Purpose**: A set of fills sums to zero.
- **Responsibility**: Assert conservation over a batch of fills.
- **In Scope**: The assertion itself.
- **Out of Scope**: Walking the book — this crate does not do that.

**Design status**: Built, as its own crate — `conserve_assert`/`fill_legs_sum`
match the proposal's own function names (its `ConserveError` has 3 variants
rather than the proposed 2; see the crate's own item doc). Full writeup
moved to [`../../module/exchange_conserve/readme.md`](../../module/exchange_conserve/readme.md),
[`../../module/exchange_conserve/docs/item/readme.md`](../../module/exchange_conserve/docs/item/readme.md).

### Statement

Without this check, the machine can silently print money. This crate closes hard problem 5 (conservation) and feature 21 (`conservation_assert`). It depends on `exchange_fill` and workstream 006's types.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:434-439` | Crate 14 in the source's Prompt 2 answer for workstream 002 |
