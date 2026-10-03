# Pitfall: STP applied after the fill is already emitted

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Checking the self-trade policy after a `Trade`/Fill has already been produced for the crossing orders.
- **In Scope**: `cross_inner`'s own ordering — the self-match key comparison relative to fill construction.

### Statement

Self-match prevention has to intercept the cross before a trade is
recorded — applying it afterward means a self-fill event has already gone
out to any consumer watching the event stream, and withdrawing it after the
fact is not the same as it never having happened.

### How this crate avoids it

Inside `cross_inner` ([`src/lib.rs`](../../src/lib.rs)), the self-match
comparison (`best.order.account == incoming.account`) runs immediately
after the best candidate is found and before `taken`/the `Trade` value are
computed at all — a self-matching pair resolves via `continue`/`break`
(withdrawing the affected side per the configured `SelfMatchPolicy`) and
the function returns to the top of the loop or exits without ever reaching
the `Trade { .. }` construction a few lines below it. No `Trade` for a
self-matching pair is ever built, so there is nothing to "withdraw after
the fact" — the ordering this pitfall warns about is structurally
impossible here, not merely avoided by discipline. `exchange_stp` (the
crate `SelfMatchPolicy` now lives in) carries no code of its own that could
run before or after a fill — its whole surface is one enum and one
name-lookup function, and its own module doc attributes the "before any
trade for the pair is built" guarantee to this crate directly. Verified
directly: [`tests/self_match_test.rs`](../../tests/self_match_test.rs)'s
`a_self_match_is_caught_even_when_the_incoming_order_is_larger` is built
exactly to catch a fill-then-check design — it uses an incoming order
larger than the self-matching resting quantity, so a comparison applied
only *after* computing a fill would still generate a partial trade for the
part that "fits" before cancelling the rest; none is generated. Reinforced
by `t09_no_trade_in_a_mixed_account_flow_carries_equal_self_match_keys`,
which asserts no `Trade` in a longer, mixed-account flow ever carries equal
self-match keys on both sides.

### Redistribution note

This pitfall was previously recorded as **out of scope** for this crate in
an earlier pass of this same effort, reasoned as "a timing invariant
spanning this crate and `exchange_stp`, not a choice internal to either
alone." Re-verified against real source for this batch: `exchange_stp`
([`../../../exchange_stp/src/lib.rs`](../../../exchange_stp/src/lib.rs))
declares only `SelfMatchPolicy` and `stp_name`, with no notion of a trade,
a loop, or a point in time at all — it cannot participate in an ordering
invariant because it has no code that runs at any point relative to
anything. `exchange_core`'s own governing algorithm doc
(`module/exchange_core/docs/algorithm/002_self_match_prevention.md`,
"Detect at crossing time, inside the match loop") attributes the detection
point to the match loop specifically, and its own Sources table cites
`exchange_match/src/lib.rs` — not `exchange_stp` — as what "implements the
key comparison." On this evidence the ordering invariant is this crate's
own, exclusively; redistributed here rather than left central.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/018_stp_applied_after_fill_emitted.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:915` | Pitfall in the source's Prompt 7 "Match policy" list |
