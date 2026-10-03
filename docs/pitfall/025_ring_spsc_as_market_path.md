# Pitfall: ring_spsc as the market path (many producers)

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Using a single-producer ring variant on a path that genuinely has many producers submitting orders.
- **In Scope**: Ring

### Statement

The exchange's inbound edge has many producers (every account submitting orders concurrently) feeding one drain — picking `ring_spsc` for that path violates its single-producer contract the moment a second submitter appears, with a failure mode that may not surface until concurrent load actually happens.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:924` | Pitfall in the source's Prompt 7 "Ring" list |
