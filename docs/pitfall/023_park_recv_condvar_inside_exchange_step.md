# Pitfall: Park, recv, or condvar inside exchange_step

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Blocking the per-tick exchange step on a park, a channel receive, or a condition variable.
- **In Scope**: Ring

### Statement

The exchange's own step function has to be a bounded, non-blocking unit of work — parking or waiting on a channel inside it stalls the entire simulation tick for however long that wait takes, which turns one book's momentary quiet into a latency spike for every other system sharing the tick.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:922` | Pitfall in the source's Prompt 7 "Ring" list |
