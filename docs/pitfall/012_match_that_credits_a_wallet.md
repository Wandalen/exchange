# Pitfall: Match that credits a wallet

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Having the matching engine itself write a balance update instead of only emitting a Fill event.
- **In Scope**: Escrow

### Statement

Workstream 002 owns the book and the match, never the wallet — crediting a balance directly from inside the match loop collapses the boundary the whole family depends on, since workstream 010 is supposed to be the only place that turns a `Fill` into a balance change.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:907` | Pitfall in the source's Prompt 7 "Escrow" list |
