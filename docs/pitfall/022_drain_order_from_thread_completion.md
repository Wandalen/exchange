# Pitfall: Drain order taken from thread completion, not drain_order

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Letting the order producer threads happen to finish in determine processing order, instead of using the ring's own defined drain order.
- **In Scope**: Ring

### Statement

Whichever producer thread happens to finish first is not a deterministic property — relying on thread-completion order instead of the ring's own total drain order means two runs of the identical input can process orders in a different sequence, breaking the determinism the whole design exists to guarantee.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:921` | Pitfall in the source's Prompt 7 "Ring" list |
