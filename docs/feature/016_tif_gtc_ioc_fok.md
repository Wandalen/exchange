# Feature: TIF: GTC, IOC, FOK

### Scope

- **Purpose**: Time-in-force as a closed set of three.
- **Responsibility**: Offer GTC, IOC, and FOK as the order's time-in-force.

**Design status**: Not implemented — see [`docs/hard_problem/020_time_in_force.md`](../hard_problem/020_time_in_force.md)'s Design status; same code-comment citation applies.

### Statement

Good-till-cancelled, immediate-or-cancel, and fill-or-kill are the three time-in-force options an order carries as data, rather than the book supporting only "rest forever."

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:324` | Feature 16 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1047` | Feature 16's English title, Prompt 9 |
