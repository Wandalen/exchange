# Hard Problem: Inbound At Aeon Edge

### Scope

- **Purpose**: Orders arrive as a drained slice, then the match runs.
- **Responsibility**: Receive orders as an already-drained slice at the aeon boundary, then run the match outside of 001's own tick.

### Statement

008 (the ring) delivers orders as an already-drained slice so that 001 is never blocked from inside the matching system. Without this boundary, a race opens up inside the simulation tick.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:237-240` | Hard problem 11 in the source's Prompt 1 answer for workstream 002 |
