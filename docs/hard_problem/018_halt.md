# Hard Problem: Halt

### Scope

- **Purpose**: Freeze the match; resting orders stay.
- **Responsibility**: Freeze matching on an instrument while leaving its resting orders in place.

**Design status**: Not addressed — no `halt_set`/`halt_clear`/`halt_is` or equivalent exists anywhere in the real crates.

### Statement

A station lockdown or circuit breaker needs to freeze matching while leaving resting orders untouched — without halt, there's no way to stop a market at all.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:272-275` | Hard problem 18 in the source's Prompt 1 answer for workstream 002 |
