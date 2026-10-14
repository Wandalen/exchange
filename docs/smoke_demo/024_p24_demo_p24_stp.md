# Smoke Demo: demo_p24_stp

### Scope

- **Purpose**: Grade phase P24.
- **Responsibility**: Confirm CancelOldest prevents a self-fill when one account sits on both sides.

### Statement

`demo_p24_stp` places orders for the same account on both sides of a cross under `CancelOldest` and confirms zero self-fills result.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:867` | Phase P24's smoke in the source's Prompt 6 answer for workstream 002 |
