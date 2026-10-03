# Stp Policy: Allow

### Scope

- **Purpose**: Let an account's own resting order and incoming order cross each other.
- **Responsibility**: One of the three self-trade policy values the source design names.
- **In Scope**: The no-op policy — match proceeds exactly as it would against any other account.
- **Out of Scope**: The other two policies' cancellation behavior.

**Design status**: Not built — the real `exchange_stp::SelfMatchPolicy`
(`src/lib.rs`) has exactly three variants, `CancelResting`/`CancelIncoming`/
`CancelBoth`, and none of them is a no-op. Self-match prevention is
unconditional in the real build — there is no way to let an account cross
itself, so this policy was not carried over at all (a stricter stance than
the proposal anticipated, not merely a renaming). Full reasoning:
[`../decisions/001_no_allow_resting_incoming_naming.md`](../decisions/001_no_allow_resting_incoming_naming.md).

### Statement

`Allow` is the policy that does nothing special: if an account's own bid and
ask happen to cross, the trade executes like any other. The proposal frames
this as the dangerous default — picking it silently, with no named policy at
all, is explicitly called out as a pitfall in Prompt 7.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:1167-1170` | Prompt 9's `stp_policy` instance list, item 1 of 3 |
