# Feature: BookStats

### Scope

- **Purpose**: Visibility into the hot path.
- **Responsibility**: Count rests, fills, and rejects for one call.

### Statement

`BookStats` counts what happened during a call — rests, fills, rejects — so the hot path isn't flying blind when something needs to be measured.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:331` | Feature 23 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1054` | Feature 23's English title, Prompt 9 |
