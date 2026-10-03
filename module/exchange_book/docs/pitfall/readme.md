# Pitfall Doc Definition

### Scope

- **Purpose**: Record the trap named for workstream 002's "Book" design area that is cleanly this crate's own price-ordering storage to avoid, so this crate's own code and tests can show it is actually avoided rather than merely cited.
- **Responsibility**: Document the pitfall, how `exchange_book` avoids it, and the test or grep that verifies the avoidance.
- **In Scope**: The one "Book" pitfall that is purely this crate's own `bids`/`asks` storage choice.
- **Out of Scope**: The other five "Book" pitfalls (006–010) — arrival/FIFO ordering within one price level is `exchange_level`'s own storage (`LevelNode`/`Level`, which this crate's `Resting` type-aliases and whose functions this crate calls), not this crate's; those stay in the central catalog pending that crate's own redistribution pass (→ [`../../../../docs/pitfall/readme.md`](../../../../docs/pitfall/readme.md)); external constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [HashMap Iteration As Price Order](001_hashmap_iteration_as_price_order.md) | Why `bids`/`asks` are plain sorted `Vec<Level>`, never a hash-keyed container | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_book/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
