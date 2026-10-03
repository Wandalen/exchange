# Pitfall Doc Definition

### Scope

- **Purpose**: Record the one trap named for workstream 002's "Match policy" design area that this crate's own variant set bears on.
- **Responsibility**: Document the pitfall, how `exchange_stp` avoids it, and the grep that verifies the avoidance.
- **In Scope**: The one central "Match policy" pitfall about self-trade having no named policy — the only one of that category this crate's own scope touches.
- **Out of Scope**: External constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Self-Trade Allowed By Default](001_self_trade_allowed_by_default.md) | Why `SelfMatchPolicy` has no `Allow` variant | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_stp/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; command grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
