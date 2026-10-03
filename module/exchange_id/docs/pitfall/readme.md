# Pitfall Doc Definition

### Scope

- **Purpose**: Record the one trap named for workstream 002's "Identity and cap" design area that bears on this crate's own `OrderId` type.
- **Responsibility**: Document the pitfall and its current, honest status — not yet structurally avoided, since no id allocator exists to guard.
- **In Scope**: The one central "Identity and cap" pitfall about id reuse — the only one of that category naming this crate's own type specifically (the other two in that category are `exchange_cap`'s and `exchange_idem`'s).
- **Out of Scope**: External constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Generation-Less Ids Reused In Same Snapshot](001_generationless_ids_reused_in_same_snapshot.md) | Why `OrderId` doesn't yet guard against reissue | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_id/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; command grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
