# item

The exposed surface of `exchange_rest`, as built.

### Scope

- **Purpose**: What this crate exports, and how it compares to the source design's exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Per-declaration files.

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `rest_place` | fn | `(&mut Book, Resting) -> bool` |
| `rest_cancel` | fn | `(&mut Book, InstrumentId, OrderId) -> Option<Resting>` |
| `RestReplaceError` | enum | `{ InstrumentMismatch, Missing, Refused, WouldTake }`, with `Display` and `Error` |
| `rest_replace` | fn | `(&mut Book, InstrumentId, OrderId, Resting) -> Result<Resting, RestReplaceError>` |

### Differs from the proposal

The source design (`core_exchange.txt:615-617`) names the same three
functions behind one `RestError { Duplicate, Missing, Full, Halted, Escrow,
Snap }`.

- **`rest_place` and `rest_cancel` have no error type.** They forward to
  `Book::insert` and `Book::cancel` and return their `bool` and
  `Option<Resting>`.
- **`RestReplaceError` holds only what a book-level replace can tell
  apart:** a replacement naming another instrument, no original to replace,
  a replacement `Book::insert` would refuse, or one that would trade. Caps,
  halt and escrow are the caller's, checked before it calls here.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:615-617` | Crate `exchange_rest`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The built shape this table describes |
