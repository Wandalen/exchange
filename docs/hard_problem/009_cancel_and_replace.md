# Hard Problem: Cancel And Replace

### Scope

- **Purpose**: Withdraw or change without leaving a ghost.
- **Responsibility**: Let a resting order be withdrawn or changed without leaving a stale remnant behind.

### Statement

Players and NPCs constantly move their quotes, so cancel and replace must remove or change an order without leaving a ghost behind. Without it, liquidity gets stuck pointing at stale orders.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:227-230` | Hard problem 9 in the source's Prompt 1 answer for workstream 002 |
