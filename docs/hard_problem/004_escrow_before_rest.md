# Hard Problem: Escrow Before Rest

### Scope

- **Purpose**: Lock first, then rest; unlock on cancel and on the remainder.
- **Responsibility**: Reserve funds before an order rests, and release the reservation on cancel or on any unfilled remainder.

### Statement

Funds must lock before an order rests, and unlock on cancel or on whatever remains unfilled, or two ticks could spend the same balance twice. Without this, the exchange oversells.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:202-205` | Hard problem 4 in the source's Prompt 1 answer for workstream 002 |
