# Pitfall Doc Definition

### Scope

- **Purpose**: Record the one trap named for workstream 002's "Identity and cap" design area that this crate's own refusal behavior bears on.
- **Responsibility**: Document the pitfall, how `exchange_idem` avoids it, and the test that verifies the avoidance.
- **In Scope**: The one central "Identity and cap" pitfall about a retry doubling a rest — the only one of that area's pitfalls this crate's own scope touches.
- **Out of Scope**: External constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Retry Inserts a Second Rest](001_retry_inserts_second_rest.md) | Why `idem_insert` refuses a repeat `OrderId` | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_idem/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
