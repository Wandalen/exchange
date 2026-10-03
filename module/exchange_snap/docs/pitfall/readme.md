# Pitfall Doc Definition

### Scope

- **Purpose**: Record the traps named for workstream 002's "Snapshot" design area, so this crate's own code and tests can show each one is actually avoided rather than merely cited.
- **Responsibility**: Document each pitfall, how `exchange_snap` avoids it, and the test or grep that verifies the avoidance.
- **In Scope**: The three pitfalls the central catalog scopes to "Snapshot" — all three map 1:1 onto this crate.
- **Out of Scope**: Pitfalls scoped to other design areas; external constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Snapshot Aliases Live Level Nodes](001_snapshot_aliases_live_level_nodes.md) | Why `RestRow` copies out plain values instead of referencing the book | 🔄 |
| 002 | [Checksum Includes HashMap Bucket Order](002_checksum_includes_hashmap_bucket_order.md) | Why `BookSnap::rows` is safe input for a later determinism checksum | 🔄 |
| 003 | [Tick Taken From Instant::now()](003_tick_taken_from_instant_now.md) | Why `tick` is a caller-supplied parameter, never a clock read | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_snap/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              3
# rows in Overview Table: 3
```
