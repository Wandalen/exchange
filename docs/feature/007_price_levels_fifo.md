# Feature: Price Levels, FIFO Inside A Level

### Scope

- **Purpose**: One price, FIFO ordering within it.
- **Responsibility**: Group resting orders by price, and order each group FIFO.

### Statement

Every price in the book is its own level, and orders resting at that level queue FIFO — first in, first matched — which is what makes price-time priority concrete rather than aspirational.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:315` | Feature 7 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1038` | Feature 7's English title, Prompt 9 |
