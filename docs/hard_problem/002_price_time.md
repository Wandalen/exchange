# Hard Problem: Price-Time

### Scope

- **Purpose**: Better price first; same price, earlier order.
- **Responsibility**: Rank resting orders by price first, then by arrival order at the same price.

### Statement

A fair, deterministic match needs the better price to go first, and the earlier order to win ties at the same price. Without price-time priority, matching degenerates into races and exploits.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:192-195` | Hard problem 2 in the source's Prompt 1 answer for workstream 002 |
