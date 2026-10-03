# Pitfall: Ring overflow as a silent drop

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Letting a full ring silently discard an order instead of surfacing it as a Reject.
- **In Scope**: Ring

### Statement

A dropped order when the ring is full is indistinguishable, to the submitter, from an order that is quietly resting — if overflow is not turned into an explicit `Reject`, a player or system believes its order is live when it was never accepted at all, and any escrow already taken against it becomes unaccounted for.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:923` | Pitfall in the source's Prompt 7 "Ring" list |
