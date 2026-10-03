# Pitfall: Reimplementing tick snap

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Writing a local price/quantity rounding routine instead of calling workstream 006's.
- **In Scope**: Money

### Statement

Snapping a price or quantity to its instrument's tick/lot grid is exact-arithmetic's job, not the exchange's — a second, hand-rolled snap implementation can disagree with 006's at the boundary, producing two different "legal" prices for the same raw input depending on which code path touched it first.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:893` | Pitfall in the source's Prompt 7 "Money" list |
