# Hard Problem: Deterministic Match

### Scope

- **Purpose**: The same book and the same input give the same fills.
- **Responsibility**: Make the same book plus the same input always produce the same fills.

### Statement

Replay, and later running on two nodes, both require that the same book plus the same input always produce the same fills. Without determinism, two runs diverge on their very first trade.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:212-215` | Hard problem 6 in the source's Prompt 1 answer for workstream 002 |
