# Feature: Fill

### Scope

- **Purpose**: One record of a match.
- **Responsibility**: Name maker, taker, price, quantity, and instrument for one executed match.

### Statement

A `Fill` records exactly who matched whom, at what price and quantity, on which instrument — the unit workstream 010 consumes to settle balances and compute fees.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:320` | Feature 12 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1043` | Feature 12's English title, Prompt 9 |
