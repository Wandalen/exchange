# Pitfall Doc Definition

### Scope

- **Purpose**: Record the traps named for workstream 002's "Money" design area that are this crate's own to avoid, so this crate's own code and tests can show each one is actually avoided rather than merely cited.
- **Responsibility**: Document each pitfall, how `exchange_conserve` avoids it, and the test or grep that verifies the avoidance.
- **In Scope**: The one "Money" pitfall that maps 1:1 onto this crate's own summation logic.
- **Out of Scope**: Pitfalls scoped to other design areas, or to other "Money" pitfalls this crate does not own (→ [`../../../../docs/pitfall/readme.md`](../../../../docs/pitfall/readme.md) for the central catalog); external constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Conservation That Adds Floats](001_conservation_that_adds_floats.md) | Why `fill_legs_sum`/`conserve_assert` never touch `f32`/`f64` | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_conserve/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
