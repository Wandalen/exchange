# Pitfall Doc Definition

### Scope

- **Purpose**: Record the pitfalls `rest_replace` must avoid.
- **Responsibility**: Document each pitfall, how `exchange_rest` avoids it, and the test that verifies it.
- **In Scope**: Central pitfall 009 (replace leaves the old rest in place), and one found while building this crate (002).
- **Out of Scope**: External constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Replace That Leaves The Old Rest In Place](001_replace_leaves_old_rest_in_place.md) | Why `rest_replace` cancels before inserting, and checks before cancelling | 🔄 |
| 002 | [A Replacement For Another Instrument](002_rest_replace_trusts_new_resting_instrument.md) | Why `rest_replace` checks `new_resting`'s instrument against its own argument | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_rest/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; command grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              2
# rows in Overview Table: 2
```
