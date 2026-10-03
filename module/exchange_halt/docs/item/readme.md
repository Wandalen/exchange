# item

The exposed surface of `exchange_halt`, as built — a consolidated index
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
| `HaltError` | enum | `{ Already }` |
| `halt_set` | fn | `(&mut InstrumentSpec) -> Result<(), HaltError>` |
| `halt_clear` | fn | `(&mut InstrumentSpec) -> Result<(), HaltError>` |
| `halt_is` | fn | `(&InstrumentSpec) -> bool` |

### Differs from the proposal

The source design's own exposed-item list
(also catalogued centrally at
[`../../../../docs/exposed_item/018_exchange_halt_items.md`](../../../../docs/exposed_item/018_exchange_halt_items.md))
names `halt_set`/`halt_clear`/`halt_is`/`HaltError { Already }` — every
function and the error shape here match exactly. The one divergence is the
dropped `exchange_book` dependency the source design's crate entry names;
see [`../decisions/001_no_exchange_book_dependency.md`](../decisions/001_no_exchange_book_dependency.md).
`halt_set`/`halt_clear` operate on `exchange_spec::InstrumentSpec` directly
(a `&mut` parameter) rather than some other handle, since that struct's own
`halted` field is the only state either function touches.
