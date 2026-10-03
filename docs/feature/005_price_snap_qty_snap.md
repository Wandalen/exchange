# Feature: price_snap, qty_snap

### Scope

- **Purpose**: Snap a submitted price or quantity onto its instrument's grid.
- **Responsibility**: Validate a price/quantity against tick/lot via workstream 006.

### Statement

`price_snap` and `qty_snap` check a submitted price or quantity against the instrument's tick/lot grid, built on workstream 006's exact arithmetic rather than reimplemented locally.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:313` | Feature 5 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1036` | Feature 5's English title, Prompt 9 |
