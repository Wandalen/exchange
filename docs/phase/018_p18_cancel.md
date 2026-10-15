# Phase: P18 — Cancel removes the rest

### Scope

- **Purpose**: Prove cancel actually removes the resting order.
- **Responsibility**: After cancel, the book no longer holds that order.

### Statement

The eighteenth phase's one new contract: resting one bid and then cancelling it leaves the book without that order — not merely unreachable by id, but actually gone.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:799` | Phase P18 in the source's Prompt 5 answer for workstream 002 |
