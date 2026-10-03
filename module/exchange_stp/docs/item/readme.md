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

The source design's own exposed-item list
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:550-552`,
also catalogued centrally at
[`../../../../docs/exposed_item/004_exchange_stp_items.md`](../../../../docs/exposed_item/004_exchange_stp_items.md))
names `Stp { Allow, CancelOldest, CancelNewest }` plus `stp_name`. `stp_name`
is built exactly as named. The enum differs: no `Allow`, the other two
renamed by role (`CancelResting`/`CancelIncoming` instead of
`CancelOldest`/`CancelNewest`), plus a third real variant (`CancelBoth`) the
proposal never named — see
[`../decisions/001_no_allow_resting_incoming_naming.md`](../decisions/001_no_allow_resting_incoming_naming.md)
for the full reasoning.

One caller-side gap, not a divergence in this crate's own surface:
`exchange_core` currently hardcodes `SelfMatchPolicy::CancelIncoming`
(`module/exchange_core/src/lib.rs:62,316,338`) rather than letting a caller
choose among the three per book.
