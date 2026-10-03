# Feature: Sequence, No Wall Clock

### Scope

- **Purpose**: Time priority from a counter, not a clock.
- **Responsibility**: Stamp resting orders with a monotonic sequence instead of wall-clock time.

### Statement

Time priority comes from a monotonic sequence number stamped on each resting order, never from `Instant`/`SystemTime` — a prerequisite for hard problem 6's determinism, since wall-clock reads differ across replays and nodes.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:336` | Feature 28 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1059` | Feature 28's English title, Prompt 9 |
