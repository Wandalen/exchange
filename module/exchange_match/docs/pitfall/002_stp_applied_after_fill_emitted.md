# Pitfall: STP applied after the fill is already emitted

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Checking the self-trade policy after a `Trade` has already been produced for the crossing orders.
- **In Scope**: `cross_inner`'s ordering — the self-match comparison relative to building the trade.

### Statement

Self-match prevention has to intercept the cross before a trade is recorded.
Applied afterwards, a self-fill has already reached whoever reads the event
stream, and withdrawing it is not the same as it never having happened.

### How this crate avoids it

In `cross_inner` ([`src/lib.rs`](../../src/lib.rs)) the comparison
`best.order.account == incoming.account` runs right after the candidate is
read, before `taken` or the `Trade` is computed. A self-matching pair leaves
through `continue` or `break` per `SelfMatchPolicy`, so no `Trade` for it is
ever built. `exchange_stp` holds only the policy enum; the ordering is this
crate's.

[`tests/self_match_test.rs`](../../tests/self_match_test.rs)'s
`a_self_match_is_caught_even_when_the_incoming_order_is_larger` uses an
incoming order larger than the self-matching resting quantity, so a
fill-then-check design would trade the part that "fits"; none is traded.
`t09_no_trade_in_a_mixed_account_flow_carries_equal_self_match_keys` checks
that no trade in a mixed-account flow carries one account on both sides.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/018_stp_applied_after_fill_emitted.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:915` | Pitfall in the source's Prompt 7 "Match policy" list |
