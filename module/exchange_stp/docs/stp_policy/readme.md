# stp_policy

One of the 26 doc entity kinds Prompt 8
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:950-985`)
names for workstream 002 — the three-value self-trade policy type the
source design proposes. Redistributed here from the central catalog since
this crate is the one place `SelfMatchPolicy` is owned.

### Scope

- **Purpose**: Catalog the source design's own three named policy values, and how each maps onto the real `SelfMatchPolicy`.
- **Responsibility**: One instance per proposal-named value — never a 4th instance for `CancelBoth`, which the proposal never named (see [`../decisions/001_no_allow_resting_incoming_naming.md`](../decisions/001_no_allow_resting_incoming_naming.md) for that addition).
- **In Scope**: The proposal's three named values, as catalog instances.
- **Out of Scope**: The real enum's own documentation (→ [`../item/readme.md`](../item/readme.md)).

### Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`001_allow.md`](001_allow.md) | Self-trade crosses normally — not built |
| [`002_canceloldest.md`](002_canceloldest.md) | Resting side withdrawn, incoming continues — held by role as `CancelResting` |
| [`003_cancelnewest.md`](003_cancelnewest.md) | Incoming side withdrawn, resting untouched — held by role as `CancelIncoming`, the one hardcoded in `exchange_core` |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exchange_stp/docs/stp_policy
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Responsibility Table: '; grep -cE '^\| \[`[0-9]{3}_' readme.md
# instances:              3
# rows in Responsibility Table: 3
```
