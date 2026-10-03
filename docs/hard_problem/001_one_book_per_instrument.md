# Hard Problem: One Book Per Instrument

### Scope

- **Purpose**: Iron and credits don't share one ladder.
- **Responsibility**: Keep every instrument's resting orders in its own book; never cross two instruments against each other.

**Design status**: Not addressed — the real `Order` struct (`exchange_types/src/lib.rs:99-111`) has no `instrument`/`InstrumentId` field at all; the real book is implicitly single-instrument.

### Statement

The game runs many markets at once, so one instrument's book must never see another's orders. Without this separation, orders from unrelated instruments could cross each other, producing false fills.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:187-190` | Hard problem 1 in the source's Prompt 1 answer for workstream 002 |
