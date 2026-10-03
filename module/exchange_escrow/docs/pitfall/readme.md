# Pitfall Doc Definition

### Scope

- **Purpose**: Record the traps named for workstream 002's "Escrow" design area that are this crate's own internal arithmetic/state-machine choices to avoid, so this crate's own code and tests can show they are actually avoided rather than merely cited.
- **Responsibility**: Document each pitfall, how `exchange_escrow` avoids it, and the test, grep, or code path that verifies the avoidance.
- **In Scope**: The two "Escrow" pitfalls that are purely this crate's own choices — arithmetic (`checked_*` vs. `saturating_*`) and the hold state machine (commit xor release, enforced via the shared `reservations` ledger).
- **Out of Scope**: The other three "Escrow" pitfalls (011–013), which describe a call-sequence invariant spanning this crate and whichever crate calls it (hold-before-rest, release-on-cancel, etc.) rather than a choice internal to this crate alone — these stay in the central catalog (→ [`../../../../docs/pitfall/readme.md`](../../../../docs/pitfall/readme.md)); external constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Saturating Add Hiding An Insolvent Hold](001_saturating_add_hiding_insolvent_hold.md) | Why every balance edit here is `checked_*`, never `saturating_*` | 🔄 |
| 002 | [Commit And Release Of The Same Hold](002_commit_and_release_of_same_hold.md) | Why a hold can only ever be resolved once, regardless of caller discipline | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_escrow/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              2
# rows in Overview Table: 2
```
