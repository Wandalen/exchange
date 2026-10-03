# Hard Problem: Tick And Lot

### Scope

- **Purpose**: An illegal price or quantity doesn't land in the book.
- **Responsibility**: Reject a price or quantity that doesn't land on the instrument's tick/lot grid.

**Design status**: Not addressed — no `InstrumentSpec`, no tick/lot concept, no snap-on-submit validation exists in the real crates.

### Statement

An instrument has a price/quantity grid; letting an illegal price or quantity land in the book produces dust. Tick and lot snapping enforces that grid before anything rests.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:222-225` | Hard problem 8 in the source's Prompt 1 answer for workstream 002 |
