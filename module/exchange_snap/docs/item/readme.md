# item

The exposed surface of `exchange_snap`, as built — a consolidated index
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
| `RestRow` | struct | `{ order : OrderId, price : Money, qty : Quantity }` |
| `BookSnap` | struct | `{ instrument : InstrumentId, tick : Money, rows : Vec<RestRow> }` |
| `snap_take` | fn | `(&Book, InstrumentId, Money) -> BookSnap` |
| `snap_len` | fn | `(&BookSnap) -> usize` |

### Differs from the proposal

The source design's own `RestRow { order, price, qty }` and
`BookSnap { instrument, tick, rows }` shapes
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:641-645`,
also catalogued centrally at
[`../../../../docs/exposed_item/020_exchange_snap_items.md`](../../../../docs/exposed_item/020_exchange_snap_items.md))
are matched exactly here — `order`/`instrument`/`tick`/`rows` are all
present, nothing added or dropped from either struct.

Two divergences do exist, both large enough to warrant their own ADRs
rather than a line here:

- No `exchange_order` dependency — [`../decisions/001_no_exchange_order_dependency.md`](../decisions/001_no_exchange_order_dependency.md).
- No `SnapError { Full }` — [`../decisions/002_no_snap_error.md`](../decisions/002_no_snap_error.md).
