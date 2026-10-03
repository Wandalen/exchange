# Pitfall: Match claiming slots itself

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Having the matching loop reach into the ring to claim slots directly, instead of only consuming an already-drained, ordered batch.
- **In Scope**: Ring

### Statement

Claiming belongs to the inbound edge, not the matcher — if match code claims ring slots itself, the matcher now has to know about ring internals (claim/publish semantics, gating), which is exactly the knowledge the inbound bridge exists to keep out of it.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:920` | Pitfall in the source's Prompt 7 "Ring" list |
