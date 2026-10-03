# Hard Problem: Partial Fills

### Scope

- **Purpose**: 10 against 3 leaves 7 in the book.
- **Responsibility**: Let a taker's quantity fill against more than one resting order, leaving any remainder resting.

### Statement

Real liquidity isn't all-or-nothing — a taker for 10 against a resting 3 should still take the 3 and leave 7 resting. Without partial fills, the book goes dead the moment sizes don't match exactly.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:197-200` | Hard problem 3 in the source's Prompt 1 answer for workstream 002 |
