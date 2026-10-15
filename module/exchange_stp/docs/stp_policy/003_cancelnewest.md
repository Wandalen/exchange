# Stp Policy: CancelNewest

### Scope

- **Purpose**: Resolve a self-trade by rejecting the incoming order instead of letting it fill.
- **Responsibility**: One of the three self-trade policy values the source design names.
- **In Scope**: Cancelling the newer (incoming) side of a same-account cross.
- **Out of Scope**: Cancelling the older (resting) side — that is `CancelOldest`'s job.

**Design status**: Held by role, not by name — `SelfMatchPolicy::CancelIncoming`
(`src/lib.rs`). In a self-match the incoming order is always the newer one,
so "cancel incoming" and "cancel newest" pick the same order. Selectable per
drain through `exchange_core::Exchange::exchange_step`. Full reasoning:
[`../decisions/001_no_allow_resting_incoming_naming.md`](../decisions/001_no_allow_resting_incoming_naming.md).

### Statement

`CancelNewest` resolves a self-trade the other direction from
`CancelOldest`: the order already resting on the book is left untouched,
and the incoming order is withdrawn before it can cross its own account's
level. Which of the two a book uses is a single configured policy, not a
per-order choice.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:1167-1170` | Prompt 9's `stp_policy` instance list, item 3 of 3 |
