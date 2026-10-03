# Smoke Demo: demo_p15_nohold

### Scope

- **Purpose**: Grade phase P15.
- **Responsibility**: Confirm a failed hold keeps the book count at zero.

### Statement

`demo_p15_nohold` forces a hold failure and confirms the book's rest count stays at zero for that order.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:849` | Phase P15's smoke in the source's Prompt 6 answer for workstream 002 |
