# item

The exposed surface of `exchange_cap`, as built — a consolidated index
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
| `BookCaps` | struct | `{ max_rests : usize, max_levels : usize }` |
| `CapError` | enum | `{ RestsFull, LevelsFull }` |
| `cap_check_rest` | fn | `(BookCaps, usize) -> Result<(), CapError>` |
| `cap_check_level` | fn | `(BookCaps, usize) -> Result<(), CapError>` |

### Matches the proposal exactly

The source design's own exposed-item list
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:587-590`,
also catalogued centrally at
[`../../../../docs/exposed_item/010_exchange_cap_items.md`](../../../../docs/exposed_item/010_exchange_cap_items.md))
names `BookCaps { max_rests, max_levels }`, `cap_check_rest`,
`cap_check_level`, and `CapError { RestsFull, LevelsFull }` — all four are
built here exactly as named, field-for-field and variant-for-variant. The
central catalog's own note that "no real crate exists for this" predates
this crate's real build and is now stale — see the thinned pointer at
[`../../../../docs/crate/010_exchange_cap.md`](../../../../docs/crate/010_exchange_cap.md).

The one thing not yet done is wiring: nothing in `exchange_book` calls either
check function yet (see this crate's own `readme.md`, "Genuinely new") — that
is a caller-side gap, not a divergence in this crate's own exposed surface.
