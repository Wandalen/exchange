# Feature: Side

### Scope

- **Purpose**: Bid and ask as one type.
- **Responsibility**: Represent which side of the book an order stands on as a dedicated type, not a bool.

### Statement

Bid and ask are the two sides of a book; a dedicated `Side` type names them directly instead of letting a bare boolean stand in for the distinction.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:310` | Feature 2 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1033` | Feature 2's English title, Prompt 9 |
