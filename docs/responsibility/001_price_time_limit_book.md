# Responsibility: Price-Time Limit Book

### Scope

- **Purpose**: State that 002's book is a price-time limit order book — the foundational claim the rest of the workstream builds on.
- **Responsibility**: The resting book orders by price first, then by arrival time within a price.
- **In Scope**: The book's own ordering discipline.
- **Out of Scope**: The matching algorithm itself (see `hard_problem/002_price_time.md`).

### Statement

002's book is a price-time limit book: at any price, the earliest-arrived order is served first, and better prices are served before worse ones. This is the first of the six responsibility statements Prompt 9 names for the workstream, and it is the precondition every other responsibility depends on — without it, "best price" and "fair order" have no fixed meaning.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:998` | First bullet of Prompt 9's `responsibility` list |
