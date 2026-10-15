# Pitfall: Resting before hold_try succeeds

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Placing an order on the book before its escrow hold is confirmed.
- **In Scope**: Escrow

### Statement

An order must not reach the book until its funds or asset are actually locked — resting it first and locking afterward opens a window where the same funds could back two live orders at once, which is exactly the double-spend this ordering exists to close.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:906` | Pitfall in the source's Prompt 7 "Escrow" list |
