# Pitfall: Cancel that frees the id but not the level node

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Releasing a cancelled order's id for reuse while its node still sits on the book.
- **In Scope**: Book

**Not cleanly `exchange_book`'s own.** `exchange_book::cancel` orchestrates
the removal, but the node storage it must stay in sync with
(`level_remove`) is `exchange_level`'s own. This is a composition invariant
spanning both crates' own storage (the id-tracking side and the node-storage
side), closer in shape to the "Escrow" boundary pitfalls (011–014) than to
a single-crate concern. `exchange_level`'s own redistribution has since
landed, confirming rather than changing this: the invariant is jointly held
by `exchange_book::cancel` and `exchange_level::level_remove` together, so
it stays central permanently rather than moving to one owner.

### Statement

Cancel has to remove the order from the book structure itself, not just mark its id available again — if the id set and the book's own storage can disagree about whether an order is still resting, a freed id can be reissued to a new order while the old, supposedly-cancelled one can still match.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:903` | Pitfall in the source's Prompt 7 "Book" list |
