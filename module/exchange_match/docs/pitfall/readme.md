# Pitfall Doc Definition

### Scope

- **Purpose**: Record the trap named for workstream 002's "Match policy" design area that is cleanly this crate's own execution mechanism to avoid, so this crate's own code and tests can show it is actually avoided rather than merely cited.
- **Responsibility**: Document the pitfall, how `exchange_match` avoids it, and the test or grep that verifies the avoidance.
- **In Scope**: The one "Match policy" pitfall whose defense (probe-then-commit) is entirely this crate's own mechanism.
- **Out of Scope**: Pitfall 017 ("IOC that rests the remainder") — this crate's own module doc states directly that it "has never inserted the incoming order's remainder itself," so resting-or-not is `exchange_core`'s/`exchange_rest`'s decision, not this crate's; pitfall 018 ("STP applied after fill emitted") — a timing invariant spanning this crate and `exchange_stp`, not a choice internal to either alone; pitfall 019 ("self-trade allowed by default") — `exchange_stp`'s own policy-API design, not this crate's. All three stay in the central catalog (→ [`../../../../docs/pitfall/readme.md`](../../../../docs/pitfall/readme.md)); external constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [FOK That Fills Part, Then Rejects](001_fok_that_fills_part_then_rejects.md) | Why a FOK order can never commit a partial fill to the real book | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_match/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
