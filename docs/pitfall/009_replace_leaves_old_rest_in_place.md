# Pitfall: Replace that leaves the old rest in place

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Implementing cancel-and-replace so the original order is still resting afterward.
- **In Scope**: Book

### Statement

A replace is supposed to remove the old order and rest the new one in its place; if the old order is left on the book, the account now has two live orders where it asked for one, doubling its exposure and reservation without its knowledge.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:902` | Pitfall in the source's Prompt 7 "Book" list |
