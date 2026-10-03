# Pitfall Doc Definition

### Scope

- **Purpose**: Record the one trap named for workstream 002's "Ring" design area that this crate's own storage choice bears on.
- **Responsibility**: Document the pitfall, how `exchange_event` avoids it, and the grep that verifies the avoidance.
- **In Scope**: The one central "Ring" pitfall about a speculative second event/fill ring — the only one of the 7 Ring pitfalls this crate's own scope touches (the other 6 are about `exchange_inbound`, which doesn't exist yet, or the book/match boundary, and stay in the central catalog).
- **Out of Scope**: External constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [A Second Fill Ring Before Asked](001_second_fill_ring_before_asked.md) | Why `event_push` et al. use a plain `Vec`, never a ring | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_event/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
