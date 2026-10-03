# Decisions Doc Definition

### Scope

- **Purpose**: Record genuine judgment calls behind this crate's design, so a later reader finds the rejected alternatives instead of re-litigating them.
- **Responsibility**: ADR-style records — the question, the options considered, the choice, and why.
- **In Scope**: Decisions about this crate's own public surface.
- **Out of Scope**: Decisions closed in another crate (→ that crate's own `decisions/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [No `LevelError`](001_no_level_error.md) | Why `Full` and `Missing` are each already handled one layer over | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_level/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
