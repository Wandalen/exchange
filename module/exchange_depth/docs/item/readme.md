# item

The exposed surface of `exchange_depth`, as built — a consolidated index
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
| `LevelView` | struct | `{ price: Money, qty: Quantity }` |
| `Depth` | struct | `{ bids: Vec<LevelView>, asks: Vec<LevelView> }` |
| `DepthError` | enum | `{ BadN }` |
| `depth_top` | fn | `(&Book, InstrumentId, usize) -> Result<Depth, DepthError>` |

### Matches the proposal exactly

The source design's own exposed-item list
(also catalogued centrally at
[`../../../../docs/exposed_item/017_exchange_depth_items.md`](../../../../docs/exposed_item/017_exchange_depth_items.md))
names `LevelView { price, qty }`, `Depth { bids, asks }`, `depth_top`, and
`DepthError { BadN }` — every field and variant here matches, with no added
or dropped surface. `BadN` is returned only for `n == 0`; a request for more
levels than rest is answered with a shorter `Vec`, not an error — a reading
of "top-N" that the proposal names but doesn't itself pin down further.

`depth_top`'s leading `InstrumentId` parameter is not a divergence — it
tracks `exchange_book::Book` going multi-instrument (one `Book` now holds
every instrument, keyed internally), so every function reading it needs to
say which instrument. The proposal predates that rework and never names the
parameter; the signature above is the real, current one.
