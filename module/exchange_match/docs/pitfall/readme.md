# Pitfall Doc Definition

### Scope

- **Purpose**: Record the "Match policy" traps whose defence is this crate's own mechanism, so its code and tests can show each is avoided.
- **Responsibility**: Document the pitfall, how `exchange_match` avoids it, and the test that verifies it.
- **In Scope**: The two "Match policy" pitfalls this crate's FOK probe and self-match ordering avoid.
- **Out of Scope**: Pitfall 017 ("IOC that rests the remainder") — `cross` never rests the incoming order, so resting is the caller's decision; pitfall 019 ("self-trade allowed by default") — `exchange_stp`'s policy design. Both stay in the central catalog (→ [`../../../../docs/pitfall/readme.md`](../../../../docs/pitfall/readme.md)). External constraints (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [FOK That Fills Part, Then Rejects](001_fok_that_fills_part_then_rejects.md) | Why a FOK order can never commit a partial fill to the real book | 🔄 |
| 002 | [STP Applied After Fill Emitted](002_stp_applied_after_fill_emitted.md) | Why the self-match check runs before a `Trade` is ever built, not after | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_match/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              2
# rows in Overview Table: 2
```
