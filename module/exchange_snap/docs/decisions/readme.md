# Decisions Doc Definition

### Scope

- **Purpose**: Record genuine judgment calls behind this crate's design, so a later reader finds the rejected alternatives instead of re-litigating them.
- **Responsibility**: ADR-style records — the question, the options considered, the choice, and why.
- **In Scope**: Decisions about this crate's own public surface.
- **Out of Scope**: Decisions closed in another crate (→ that crate's own `decisions/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [No `exchange_order` Dependency](001_no_exchange_order_dependency.md) | Why the source design's first named dependency is not taken | 🔄 |
| 002 | [No `SnapError`](002_no_snap_error.md) | Why `snap_take` is infallible despite the proposal naming an error type | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_snap/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              2
# rows in Overview Table: 2
```
