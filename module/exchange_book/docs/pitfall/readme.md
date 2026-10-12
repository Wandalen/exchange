# Pitfall Doc Definition

### Scope

- **Purpose**: Record the traps named for workstream 002's "Book" design area that are cleanly this crate's own price-ordering storage and API shape to avoid, so this crate's own code and tests can show each is actually avoided rather than merely cited.
- **Responsibility**: Document the pitfall, how `exchange_book` avoids it, and the test or grep that verifies the avoidance.
- **In Scope**: The two "Book" pitfalls that are purely this crate's own `bids`/`asks` storage choice and `best`/`consume_best` indexing.
- **Out of Scope**: "Book" pitfalls 006, 007, 009 and 010, about arrival order within one level or about cancel and replace, which stay in the central catalog (→ [`../../../../docs/pitfall/readme.md`](../../../../docs/pitfall/readme.md)); external constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [HashMap Iteration As Price Order](001_hashmap_iteration_as_price_order.md) | Why `bids`/`asks` are a sorted `VecDeque<Level>`, never a hash-keyed container | 🔄 |
| 002 | [Crossing Worse Level While Better Has Qty](002_crossing_worse_level_while_better_has_qty.md) | Why `best`/`consume_best` can never reach a level but the current-front one | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_book/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              2
# rows in Overview Table: 2
```
