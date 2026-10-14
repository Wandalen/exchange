# item

The exposed surface of `exchange_stp`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice; see this
family's own `docs/research/` for why a fuller `item_des.rulebook.md`
per-declaration split is deferred).

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a short note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above).

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `SelfMatchPolicy` | enum | `{ CancelResting, CancelIncoming, CancelBoth }` |
| `stp_name` | fn | `(SelfMatchPolicy) -> &'static str` |

### Diverges from the proposal

The source design's exposed-item list
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:550-552`,
catalogued at
[`../../../../docs/exposed_item/004_exchange_stp_items.md`](../../../../docs/exposed_item/004_exchange_stp_items.md))
names `Stp { Allow, CancelOldest, CancelNewest }` plus `stp_name`. `stp_name`
is built as named. The enum differs: no `Allow`, the other two renamed by
role (`CancelResting`/`CancelIncoming`), plus `CancelBoth`, which the
proposal never named — see
[`../decisions/001_no_allow_resting_incoming_naming.md`](../decisions/001_no_allow_resting_incoming_naming.md).
