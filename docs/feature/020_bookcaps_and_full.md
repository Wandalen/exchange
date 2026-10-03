# Feature: BookCaps And Full

### Scope

- **Purpose**: A named limit, and a named error when it's hit.
- **Responsibility**: Cap resting orders and levels; report Full rather than drop or grow unbounded.

### Statement

`BookCaps` names the limits on resting orders and levels; hitting one returns a `Full` error to the caller instead of silently dropping the order or letting the book grow without bound.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:328` | Feature 20 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1051` | Feature 20's English title, Prompt 9 |
