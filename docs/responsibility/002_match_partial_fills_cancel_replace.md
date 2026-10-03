# Responsibility: Match, Partial Fills, Cancel, Replace

### Scope

- **Purpose**: State that 002 owns the full lifecycle of a resting order, not just its initial placement.
- **Responsibility**: Matching against the book, partial fills, cancellation, and replacement are all 002's responsibility.
- **In Scope**: `exchange_match`'s crossing loop, cancel and replace operations on the book.
- **Out of Scope**: What happens to funds on a fill (escrow's job, see responsibility 003).

### Statement

002 owns the complete resting-order lifecycle: an order can match (fully or partially), be cancelled, or be replaced, and all three are first-class responsibilities of this workstream rather than incidental behavior. Partial fills in particular require the book to track and expose a remaining quantity rather than treating every order as all-or-nothing.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:999` | Second bullet of Prompt 9's `responsibility` list |
