# Feature: InstrumentSpec

### Scope

- **Purpose**: One record for an instrument's tradeable grid.
- **Responsibility**: Name an instrument's tick, lot, halt flag, and asset pair.

**Design status**: Not built — no `InstrumentSpec` type exists in the real crates.

### Statement

An instrument needs its own tick size, lot size, halt flag, and asset pair recorded somewhere a submission can be checked against — `InstrumentSpec` is that record.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:312` | Feature 4 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1035` | Feature 4's English title, Prompt 9 |
