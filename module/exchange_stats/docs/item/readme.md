# item

The exposed surface of `exchange_stats`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice; see this
family's own `docs/research/` for why a fuller `item_des.rulebook.md`
per-declaration split is deferred).

### Scope

- **Purpose**: What this crate actually exports, and how it differs from the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a short note where the real shape diverges from the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above).

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `BookStats` | struct | `{ rests: u64, fills: u64, rejects: u64, cancels: u64 }` |
| `stats_zero` | fn | `() -> BookStats` |
| `stats_rest_add` | fn | `(&mut BookStats, u64)` |
| `stats_fill_add` | fn | `(&mut BookStats, u64)` |
| `stats_reject_add` | fn | `(&mut BookStats, u64)` |
| `stats_cancel_add` | fn | `(&mut BookStats, u64)` |
| `stats_snapshot` | fn | `(&BookStats) -> BookStats` |

### Differs from the proposal

The source design's own `BookStats` shape
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:648`) is
`{ rests, fills, rejects, cancels }` — matched exactly here. Its exposed-item
*function* list (`core_exchange.txt:647-650`, also catalogued centrally at
[`../../../../docs/exposed_item/021_exchange_stats_items.md`](../../../../docs/exposed_item/021_exchange_stats_items.md))
names only `stats_zero`/`stats_fill_add`/`stats_reject_add`/`stats_snapshot`,
leaving both `rests` and `cancels` with no incrementing function — so
`stats_rest_add` and `stats_cancel_add` are both added here, for symmetry
with `stats_fill_add`/`stats_reject_add` (see this crate's own module doc
comment). No dependency on `exchange_id`, for the reason in
[`../decisions/001_no_exchange_id_dependency.md`](../decisions/001_no_exchange_id_dependency.md).
