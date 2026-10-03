# Exposed Item: exchange_conserve

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_conserve`.
- **Responsibility**: Asserting that a set of fills sums to zero.

**Design status**: Built, as its own crate — `fill_legs_sum`/`conserve_assert`
match the proposal's own names; `ConserveError` has 3 variants
(`Notional`, `Overflow`, `Unbalanced` — the proposal's `NotZero` renamed,
plus a `Notional` variant the proposal didn't name) rather than the
proposed 2. Full comparison, including why this crate's existence does not
contradict `Escrow::total_cash()`/`total_asset()` serving a *different*,
global-balance-sheet check, moved to
[`../../module/exchange_conserve/docs/item/readme.md`](../../module/exchange_conserve/docs/item/readme.md).

### Statement

Prompt 3 specifies a standalone crate asserting that one set of fill legs sums to zero, with its own error type. The real build has no such crate; conservation is instead checked by summing total balances before and after an operation in `exchange_escrow`, reusing that crate's own error type rather than introducing a dedicated one.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:610-613` | Crate `exchange_conserve`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
