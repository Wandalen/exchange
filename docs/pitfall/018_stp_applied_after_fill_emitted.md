# Pitfall: STP applied after the fill is already emitted

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Checking the self-trade policy after a Fill event has already been produced for the crossing orders.
- **In Scope**: Match policy

### Statement

Self-match prevention has to intercept the cross before a trade is recorded — applying it afterward means a self-fill event has already gone out to any consumer watching the event stream, and withdrawing it after the fact is not the same as it never having happened.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:915` | Pitfall in the source's Prompt 7 "Match policy" list |
