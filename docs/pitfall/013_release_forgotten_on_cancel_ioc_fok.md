# Pitfall: Release forgotten on cancel, IOC leftover, or FOK reject

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Leaving a hold locked after the order it backed no longer needs it.
- **In Scope**: Escrow

### Statement

Three exit paths each need their hold released exactly once — a plain cancel, an IOC order's unfilled remainder, and a rejected FOK — and missing any one of them leaves funds permanently locked against an order that no longer exists, which looks like a leak rather than a crash and is easy to miss in testing.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:908` | Pitfall in the source's Prompt 7 "Escrow" list |
