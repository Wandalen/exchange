# Pitfall Doc Definition

### Scope

- **Purpose**: Record the traps whose failures leave every order-level check passing, so the gap is visible before an implementation walks into one and the conservation audit reports it months later without attribution.
- **Responsibility**: Document `exchange_core`'s own traps — the trap, the failure it produces, and the mitigation or the named undecided choice.
- **In Scope**: Failures that are silent at the moment of fault and surface far from their cause — ledger corruption first.
- **Out of Scope**: Traps in the intake mechanism itself, a separate contract; external constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Balance Lost Update Under Concurrent Settlement](001_balance_lost_update.md) | Two settlements overwrite one another with no error, no illegal state, and no failing count — and the audit that finds it cannot attribute it | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_core/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
