# Smoke Demo: demo_p27_snap

### Scope

- **Purpose**: Grade phase P27.
- **Responsibility**: Confirm a snapshot survives a live cancel unchanged.

### Statement

`demo_p27_snap` takes a snapshot, cancels the live order it captured, and confirms the snapshot still shows that row.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:873` | Phase P27's smoke in the source's Prompt 6 answer for workstream 002 |
