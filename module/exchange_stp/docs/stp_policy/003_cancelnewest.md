# Stp Policy: CancelNewest

### Scope

- **Purpose**: Resolve a self-trade by rejecting the incoming order instead of letting it fill.
- **Responsibility**: One of the three self-trade policy values the source design names.
- **In Scope**: Cancelling the newer (incoming) side of a same-account cross.
- **Out of Scope**: Cancelling the older (resting) side — that is `CancelOldest`'s job.

**Design status**: Held by role, and the one actually wired up — the real
`exchange_stp::SelfMatchPolicy::CancelIncoming` variant (`src/lib.rs`) is
this policy's functional equivalent: the incoming order is always the
*newer* one in a self-match, so "cancel incoming" and "cancel newest" pick
the same order. `exchange_core::Exchange::submit`
(`module/exchange_core/src/lib.rs:316,338`) hardcodes exactly this variant —
its own doc comment at line 62 states the policy "is hardcoded below to
`SelfMatchPolicy::CancelIncoming`," not yet caller-configurable. A third real
variant, `CancelBoth` (withdraw both remainders), has no counterpart in this
proposal's three named policies at all. Full reasoning:
[`../decisions/001_no_allow_resting_incoming_naming.md`](../decisions/001_no_allow_resting_incoming_naming.md).

(Superseded note: this file previously cited `exchange_match/src/lib.rs` as
the owning module and different line numbers — `SelfMatchPolicy` has since
moved to this crate, `exchange_stp`, which `exchange_match` now re-exports
from.)

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
