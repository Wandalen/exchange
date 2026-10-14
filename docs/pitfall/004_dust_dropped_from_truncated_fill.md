# Pitfall: Dust dropped because a fill qty was truncated

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Losing a sub-unit remainder when a fill quantity is truncated instead of exactly split.
- **In Scope**: Money

### Statement

When a partial fill's quantity is computed and then truncated rather than split exactly, the leftover fraction simply vanishes from both sides of the trade — a small, silent leak that conservation checks are specifically meant to catch, so letting it happen defeats the reason that check exists.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:895` | Pitfall in the source's Prompt 7 "Money" list |
