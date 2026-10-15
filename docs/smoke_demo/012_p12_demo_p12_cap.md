# Smoke Demo: demo_p12_cap

### Scope

- **Purpose**: Grade phase P12.
- **Responsibility**: Confirm a third rest against a cap of 2 is refused.

### Statement

`demo_p12_cap` rests two orders against a cap of 2, then attempts a third and confirms it is refused with `ok_full` rather than silently accepted.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:843` | Phase P12's smoke in the source's Prompt 6 answer for workstream 002 |
