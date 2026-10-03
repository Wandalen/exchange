# Pitfall Doc Definition

### Scope

- **Purpose**: Record the two traps named for workstream 002's "Process" design area that bear on this crate's own existence as a dedicated, per-phase smoke suite.
- **Responsibility**: Document each pitfall, how this crate's structure partially avoids it, and the honest gap (enforcement, not just existence) that remains.
- **In Scope**: The two central "Process" pitfalls about per-phase coverage discipline — the only two of that category naming a testing-process concern this crate is the actual home of.
- **Out of Scope**: External constraints this crate absorbs (→ [`../workaround/`](../workaround/readme.md)); the third "Process" pitfall (`035`, building a wallet inside workstream 002) stays central — it's an `exchange_escrow`-vs-workstream-010 scope boundary, not a testing-process concern of this crate's.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [P30 Green While Earlier Phases Red](001_p30_green_while_earlier_phases_red.md) | Why per-phase binaries exist separately from the wall | 🔄 |
| 002 | [Escrow Stub Always Succeeds, No Failing-Hold Phase](002_escrow_stub_always_succeeds_no_failing_hold_phase.md) | Why `demo_p15_nohold` exists independent of the wall's stub | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/smoke_exchange_phases/docs/pitfall
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; command grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              2
# rows in Overview Table: 2
```
