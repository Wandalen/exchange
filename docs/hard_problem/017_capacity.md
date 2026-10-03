# Hard Problem: Capacity

### Scope

- **Purpose**: A limit on resting orders; Full is an error, not a silent drop.
- **Responsibility**: Enforce a cap on resting orders; report Full as an error rather than silently dropping the order.

### Statement

The same capacity discipline used in workstreams 001 and 003 applies here: a cap on resting orders, and Full as an explicit error rather than orders quietly vanishing.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:267-270` | Hard problem 17 in the source's Prompt 1 answer for workstream 002 |
