# item

The exposed surface of `exchange_spec`, as built — a consolidated index
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
| `AssetId` | struct | `(pub u32)` |
| `InstrumentSpec` | struct | `{ id, base, quote, tick: Tick, lot: Lot, halted: bool }` |
| `SpecError` | type alias | `= exact_arith::SnapError` |
| `spec_new` | fn | `(InstrumentId, AssetId, AssetId, Price, Quantity) -> Result<InstrumentSpec, SpecError>` |
| `spec_halted_is` | fn | `(&InstrumentSpec) -> bool` |
| `price_snap` | fn | `(&InstrumentSpec, Price) -> Result<Price, SpecError>` |
| `qty_snap` | fn | `(&InstrumentSpec, Quantity) -> Result<Quantity, SpecError>` |

### Differs from the proposal

The source design's exposed-item list (`core_exchange.txt:554-559`,
catalogued at
[`../../../../docs/exposed_item/005_exchange_spec_items.md`](../../../../docs/exposed_item/005_exchange_spec_items.md))
names `AssetId(u32)`, `InstrumentSpec{id,base,quote,tick,lot,halted}`,
`spec_new`/`spec_halted_is`, `price_snap`/`qty_snap`, and
`SpecError{ZeroTick,ZeroLot,Snap}` — all built as named. `SpecError` is an
alias of `exact_arith::SnapError` rather than its own enum, so its third
variant is `Overflow`, not `Snap`.
