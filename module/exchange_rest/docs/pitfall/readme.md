# Pitfall Doc Definition

### Scope

- **Purpose**: Record the one central "Book" pitfall that is cleanly this crate's own `rest_replace` ordering to avoid, so this crate's own code and tests can show it is actually avoided rather than merely cited.
- **Responsibility**: Document the pitfall, how `exchange_rest` avoids it, and the test (plus mutation check) that verifies the avoidance.
- **In Scope**: Central pitfall `009` (replace leaves the old rest in place) — confirmed this session to be `rest_replace`'s own cancel/insert/rollback concern, not `exchange_inbound`'s (whose `inbound_apply` only delegates to it, see that crate's own `docs/pitfall/readme.md`).
- **Out of Scope**: External constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Replace That Leaves The Old Rest In Place](001_replace_leaves_old_rest_in_place.md) | Why `rest_replace` cancels before inserting, with rollback on refusal | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_rest/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; command grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
