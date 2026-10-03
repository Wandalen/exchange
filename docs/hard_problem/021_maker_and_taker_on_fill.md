# Hard Problem: Maker And Taker On Fill

### Scope

- **Purpose**: Every fill names both sides and the price.
- **Responsibility**: Name both the maker and the taker, and the execution price, on every fill.

### Statement

Workstream 010 computes fees and logs settlement from the fill alone, so every fill must name both the maker and taker and the price it executed at — otherwise settlement doesn't know who to credit.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:287-290` | Hard problem 21 in the source's Prompt 1 answer for workstream 002 |
