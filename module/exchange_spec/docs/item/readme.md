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

The source design's own exposed-item list
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:554-559`,
also catalogued centrally at
[`../../../../docs/exposed_item/005_exchange_spec_items.md`](../../../../docs/exposed_item/005_exchange_spec_items.md) —
whose own "Not built" Design status line is now stale, see below) names
`AssetId(u32)`, `InstrumentSpec{id,base,quote,tick,lot,halted}`,
`spec_new`/`spec_halted_is`, `price_snap`/`qty_snap`, and
`SpecError{ZeroTick,ZeroLot,Snap}` — every struct field, and every function
signature, matches exactly. The one real divergence: `SpecError` is a type
alias for `exact_arith::SnapError` (`exact_snap/src/lib.rs:29-37`) rather
than its own enum, so it inherits that type's exact 3 variants —
`ZeroTick`/`ZeroLot` match the proposal's own names; the third is named
`Overflow` there, not the proposal's `Snap`. Not worth a dedicated ADR: the
type-alias choice itself is already explained in this crate's own module
doc comment (one piece of arithmetic should have one source of truth), and
the one-variant naming difference is inherited from a dependency this crate
doesn't control, not a decision made here.

### Central doc staleness found while redistributing

Both central `docs/crate/005_exchange_spec.md` and
`docs/exposed_item/005_exchange_spec_items.md` still say "Not built" —
stale since this crate was completed earlier this session. Thinned to
pointers as part of this redistribution pass rather than left stale.
