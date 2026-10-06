# item

The exposed surface of `exchange_tif`, as built — a consolidated index
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
| `Tif` | enum | `{ Gtc, Ioc, Fok }` |
| `tif_rests` | fn | `(Tif) -> bool` |
| `tif_requires_full` | fn | `(Tif) -> bool` |

### Matches the proposal exactly

The source design's exposed-item list
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:546-548`,
catalogued at
[`../../../../docs/exposed_item/003_exchange_tif_items.md`](../../../../docs/exposed_item/003_exchange_tif_items.md))
names `Tif { Gtc, Ioc, Fok }` plus `tif_rests`/`tif_requires_full` — all
three are built as named. Which callers consult which function:
[`../../readme.md`](../../readme.md), "Wiring".
