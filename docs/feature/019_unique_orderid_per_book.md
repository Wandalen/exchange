# Feature: Unique OrderId Per Book

### Scope

- **Purpose**: One OrderId, one order, per book.
- **Responsibility**: Refuse a second submission under an OrderId already live in the book.

### Statement

A book tracks which OrderIds are currently live and refuses a duplicate submission under the same id, which is what makes a retried send safe rather than a second resting order.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:327` | Feature 19 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1050` | Feature 19's English title, Prompt 9 |
