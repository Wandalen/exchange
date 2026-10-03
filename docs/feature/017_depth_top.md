# Feature: depth_top

### Scope

- **Purpose**: Top-N book depth in one call.
- **Responsibility**: Return the best N price levels per side without a full walk.

**Design status**: Not built — see [`docs/hard_problem/013_depth.md`](../hard_problem/013_depth.md).

### Statement

`depth_top(n)` answers "what are the best N levels right now" directly, rather than making every UI or NPC caller walk the whole ladder itself.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:325` | Feature 17 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1048` | Feature 17's English title, Prompt 9 |
