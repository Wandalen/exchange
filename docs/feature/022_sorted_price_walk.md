# Feature: Sorted Price Walk

### Scope

- **Purpose**: Walk prices in sorted order, not map iteration order.
- **Responsibility**: Traverse price levels from the best price outward, by sort order.

### Statement

Matching and depth both walk price levels starting from the best price, in sorted order — never by iterating a hash map, whose order is an implementation accident rather than a price ordering.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:330` | Feature 22 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1053` | Feature 22's English title, Prompt 9 |
