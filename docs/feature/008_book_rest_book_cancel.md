# Feature: book_rest, book_cancel

### Scope

- **Purpose**: Place a resting order; remove one.
- **Responsibility**: Insert an order into the book, and remove one by id.

### Statement

`book_rest` places an order onto its price level; `book_cancel` removes a resting order by id — the two primitive mutations every higher-level operation (replace, match) builds from.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:316` | Feature 8 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1039` | Feature 8's English title, Prompt 9 |
