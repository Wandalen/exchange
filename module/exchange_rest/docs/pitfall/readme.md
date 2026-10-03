# Pitfall Doc Definition

### Scope

- **Purpose**: Record pitfalls this crate's own `rest_replace` either avoids or, honestly, does not yet guard against.
- **Responsibility**: Document each pitfall, whether `exchange_rest` avoids it, and the test (or deliberate absence of one) that backs that claim.
- **In Scope**: Central pitfall `009` (replace leaves the old rest in place) — confirmed this session to be `rest_replace`'s own cancel/insert/rollback concern, not `exchange_inbound`'s (whose `inbound_apply` only delegates to it, see that crate's own `docs/pitfall/readme.md`); plus one pitfall found during this session's work with no central-catalog origin (002).
- **Out of Scope**: External constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Replace That Leaves The Old Rest In Place](001_replace_leaves_old_rest_in_place.md) | Why `rest_replace` cancels before inserting, with rollback on refusal | 🔄 |
| 002 | [`rest_replace` Trusts `new_resting`'s Own Instrument](002_rest_replace_trusts_new_resting_instrument.md) | An open gap: no cross-check that `new_resting.order.instrument` agrees with the `instrument` argument | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_rest/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; command grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              2
# rows in Overview Table: 2
```
