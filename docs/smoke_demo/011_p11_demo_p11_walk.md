# Smoke Demo: demo_p11_walk

### Scope

- **Purpose**: Grade phase P11.
- **Responsibility**: Confirm price walk order is 1.00 then 0.95, not map iteration order.

### Statement

`demo_p11_walk` walks the book's prices and confirms the order is 1.00 then 0.95 — sorted, not map-iteration order.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:841` | Phase P11's smoke in the source's Prompt 6 answer for workstream 002 |
