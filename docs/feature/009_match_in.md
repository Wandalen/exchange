# Feature: match_in

### Scope

- **Purpose**: Cross a taker against the opposite ladder.
- **Responsibility**: Produce fills and a remainder from an incoming order against the book.

### Statement

`match_in` crosses an incoming order against the opposite side of the book, producing zero or more fills plus whatever quantity remains — the single entry point into the matching algorithm.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:317` | Feature 9 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1040` | Feature 9's English title, Prompt 9 |
