# Stp Policy: CancelOldest

### Scope

- **Purpose**: Resolve a self-trade by withdrawing the resting order instead of letting it fill.
- **Responsibility**: One of the three self-trade policy values the source design names.
- **In Scope**: Cancelling the older (resting) side of a same-account cross.
- **Out of Scope**: Cancelling the newer (incoming) side — that is `CancelNewest`'s job.

**Design status**: Held by role, not by name — the real
`exchange_stp::SelfMatchPolicy::CancelResting` variant (`src/lib.rs`) is this
policy's functional equivalent: in a self-match, the resting order is always
the *older* one (it arrived first, which is why it was resting), so "cancel
resting" and "cancel oldest" pick the same order. The real enum names the
variant by the order's *role* in the cross rather than its arrival time.
Not the policy actually in effect, though — `exchange_core` hardcodes
`CancelIncoming` (see `003_cancelnewest.md`), not this one. Full reasoning:
[`../decisions/001_no_allow_resting_incoming_naming.md`](../decisions/001_no_allow_resting_incoming_naming.md).

### Statement

`CancelOldest` resolves a self-trade by pulling the resting order off the
book rather than crossing it — the incoming order then continues matching
against the next-best level as if the self-owned level were never there.
This is the policy the project's own wall-smoke scenario picks to
demonstrate "no self-fill."

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:1167-1170` | Prompt 9's `stp_policy` instance list, item 2 of 3 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:693` | Named as the policy under test in the proposed wall smoke |
