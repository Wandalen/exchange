# Exposed Item: exchange_conserve

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_conserve`.
- **Responsibility**: Asserting that a set of fills sums to zero.

**Design status**: Built — `conserve_assert` as proposed; `fill_legs_sum`
not built, `exact_arith::money_sum_assert_zero` doing the sum; `ConserveError`
with `Notional`, `Overflow` and `Unbalanced` (the proposal's `NotZero`).
Built-vs-proposed comparison:
[`../../module/exchange_conserve/docs/item/readme.md`](../../module/exchange_conserve/docs/item/readme.md).

### Statement

Prompt 3 specifies a standalone crate asserting that one set of fill legs sums to zero, with its own error type.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:610-613` | Crate `exchange_conserve`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
