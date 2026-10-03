# Neighbor Contract: 004 Does Not Stamp Priority

### Scope

- **Purpose**: Rule out wall-clock time as any part of 002's price-time priority.
- **Responsibility**: Priority within a price level comes from workstream 002's own `Sequence`, never from workstream 004 (time).
- **In Scope**: `exchange_seq`'s monotonic counter as the sole priority stamp.
- **Out of Scope**: `Instant`, `SystemTime`, or any wall-clock read anywhere in the book or matcher.

### Statement

This is a negative contract: workstream 004 (time) contributes nothing to 002. Price-time priority is ordered by a monotonic sequence number 002 assigns itself, specifically so replay and determinism never depend on when a tick happened to run — see `pitfall/` category "Book" for the concrete failure mode (`Instant` or `SystemTime` as time priority).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1092` | Fifth bullet of Prompt 9's `neighbor_contract` list |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:147` | "004 час. Пріоритет — sequence, не SimTime" (004 time: priority is sequence, not SimTime) |
