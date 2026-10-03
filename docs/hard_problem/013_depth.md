# Hard Problem: Depth

### Scope

- **Purpose**: Top-N without walking every resting order.
- **Responsibility**: Answer top-N book depth without walking every resting order.

**Design status**: Not addressed — no `depth_top` function or equivalent exists anywhere in the real crates.

### Statement

UI and NPC consumers need top-N depth without walking every resting order, or every quote costs O(n).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:247-250` | Hard problem 13 in the source's Prompt 1 answer for workstream 002 |
