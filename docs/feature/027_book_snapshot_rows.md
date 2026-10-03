# Feature: Book Snapshot Rows

### Scope

- **Purpose**: A copyable view of the book, tick supplied by the caller.
- **Responsibility**: Produce plain snapshot rows of resting orders, tick taken from the caller.

### Statement

A book snapshot is a set of plain rows copied out of the live book, with the tick supplied by whoever calls for the snapshot rather than read from a wall clock — matching hard problem 22's save/replay/extract need.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:335` | Feature 27 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1058` | Feature 27's English title, Prompt 9 |
