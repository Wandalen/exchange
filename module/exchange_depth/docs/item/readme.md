# item

The exposed surface of `exchange_depth`, as built.

### Scope

- **Purpose**: What this crate exports, and how it compares to the source design's exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Per-declaration files.

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `LevelView` | struct | `{ price: Price, qty: Quantity }` |
| `Depth` | struct | `{ bids: Vec<LevelView>, asks: Vec<LevelView> }` |
| `DepthError` | enum | `{ BadN }`, with `Display` and `Error` |
| `depth_top` | fn | `(&Book, InstrumentId, usize) -> Result<Depth, DepthError>` |

### Matches the proposal

The source design (`core_exchange.txt:625-629`) names `LevelView { price,
qty }`, `Depth { bids, asks }`, `depth_top` and `DepthError { BadN }`; every
field and variant is here. `BadN` is returned only for `n == 0` — an `n`
above what rests returns fewer rows. `depth_top` takes the instrument because
one `Book` holds every instrument.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:625-629` | Crate `exchange_depth`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The built shape this table describes |
