# Pitfall Doc Definition

### Scope

- **Purpose**: Record the one trap named for workstream 002's "Identity and
  cap" design area that this crate's own return value bears on.
- **Responsibility**: Document the pitfall, how `exchange_cap` avoids it, and
  the test that verifies the avoidance.
- **In Scope**: The one central "Identity and cap" pitfall about capacity
  refusal being silent — the other two in that list (retry inserts a second
  rest, id reuse) belong to `exchange_idem` and `exchange_id`.
- **Out of Scope**: External constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Full Drops the Order and Returns Ok](001_full_drops_order_returns_ok.md) | Why both cap checks return `Result`, never a silent `Ok` | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_cap/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
