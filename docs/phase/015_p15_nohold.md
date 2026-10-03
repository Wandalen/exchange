# Phase: P15 — Failed hold keeps order out of the book

### Scope

- **Purpose**: Prove a failed reservation never reaches the book.
- **Responsibility**: If `hold_try` fails, the order is not resting anywhere.

### Statement

The fifteenth phase's one new contract: when the hold fails, the order never enters the book at all — the book's rest count stays at zero for that order.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:796` | Phase P15 in the source's Prompt 5 answer for workstream 002 |
