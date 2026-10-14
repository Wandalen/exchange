# item

The exposed surface of `exchange_snap`, as built.

### Scope

- **Purpose**: What this crate exports, and how it differs from the source design's exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a note where the shape diverges from the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Per-declaration files.

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `RestRow` | struct | `{ order : OrderId, side : Side, price : Price, qty : Quantity }` |
| `BookSnap` | struct | `{ instrument : InstrumentId, tick : u64, rows : Vec<RestRow> }` |
| `snap_take` | fn | `(&Book, InstrumentId, u64) -> BookSnap` |
| `snap_len` | fn | `(&BookSnap) -> usize` |

### Differs from the proposal

The source design (`core_exchange.txt:641-645`) names `RestRow { order,
price, qty }`, `BookSnap { instrument, tick, rows }`, `snap_take`, `snap_len`
and `SnapError { Full }`.

- **`RestRow::side`** is added. `rows` lists bids then asks with no marker
  between them, so without it a one-sided or crossed book reads two ways.
- **`tick` is a `u64`**, the caller's step count. The proposal gives it no
  type.

Two divergences have their own decisions:

- No `exchange_order` dependency — [`../decisions/001_no_exchange_order_dependency.md`](../decisions/001_no_exchange_order_dependency.md).
- No `SnapError { Full }` — [`../decisions/002_no_snap_error.md`](../decisions/002_no_snap_error.md).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:641-645` | Crate `exchange_snap`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The built shape this table describes |
