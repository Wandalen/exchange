# Feature: Overflow → Reject

### Scope

- **Purpose**: A full ring turns into a named error, not nothing.
- **Responsibility**: Convert a ring-full condition into a Reject for the order that didn't fit.

### Statement

When the inbound ring is full, the order that couldn't be published becomes a `Reject`, not a silently dropped submission — the concrete mechanism behind hard problem 24.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:338` | Feature 30 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1061` | Feature 30's English title, Prompt 9 |
