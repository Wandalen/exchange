# Item Doc Definition

### Scope

- **Purpose**: The full exposed surface this crate declares, as built vs. as the source proposal specified it.
- **Responsibility**: One consolidated table plus a comparison against `core_exchange.txt`.
- **In Scope**: `HaltError`, `halt_set`, `halt_clear`, `halt_is`.
- **Out of Scope**: Build status prose for the whole crate (→ [`../../readme.md`](../../readme.md)); the central summary (→ [`../../../../docs/crate/018_exchange_halt.md`](../../../../docs/crate/018_exchange_halt.md)).

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `HaltError` | enum | `{ Already }`, with `Display` and `Error` |
| `halt_set` | fn | `(&mut InstrumentSpec) -> Result<(), HaltError>` |
| `halt_clear` | fn | `(&mut InstrumentSpec) -> Result<(), HaltError>` |
| `halt_is` | fn | `(&InstrumentSpec) -> bool` |

### Matches the proposal, minus a dependency

The source design's exposed-item list (`core_exchange.txt:631-633`,
catalogued at
[`../../../../docs/exposed_item/018_exchange_halt_items.md`](../../../../docs/exposed_item/018_exchange_halt_items.md))
names `halt_set`/`halt_clear`/`halt_is` and `HaltError { Already }` — all
built as named, on `&InstrumentSpec`/`&mut InstrumentSpec`, whose `halted`
field is the only state they touch. The crate entry's second dependency,
`exchange_book`, is not taken — see
[`../decisions/001_no_exchange_book_dependency.md`](../decisions/001_no_exchange_book_dependency.md).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:458-463` | Crate 18 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:631-633` | Crate `exchange_halt`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
