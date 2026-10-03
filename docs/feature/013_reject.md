# Feature: Reject

### Scope

- **Purpose**: An order refusal that names why.
- **Responsibility**: Carry the order and a closed-set reason for every refusal.

### Statement

A `Reject` names the order that was refused and a closed-set reason — never a bare failure — so a caller can branch on why rather than parse prose.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:321` | Feature 13 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1044` | Feature 13's English title, Prompt 9 |
