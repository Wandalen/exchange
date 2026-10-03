# item

The exposed surface of `exchange_rest`, as built — a consolidated index
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
| `rest_place` | fn | `(&mut Book, Resting) -> bool` |
| `rest_cancel` | fn | `(&mut Book, InstrumentId, OrderId) -> Option<Resting>` |
| `RestReplaceError` | enum | `{ Missing, Refused }` |
| `rest_replace` | fn | `(&mut Book, InstrumentId, OrderId, Resting) -> Result<Resting, RestReplaceError>` |

### Differs from the proposal

The source design's own exposed-item list for `exchange_rest`
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:615-617`,
also `:1238-1239`, catalogued centrally at
[`../../../../docs/exposed_item/015_exchange_rest_items.md`](../../../../docs/exposed_item/015_exchange_rest_items.md))
names a `rest_place`/`rest_cancel`/`rest_replace` trio behind one dedicated
`RestError { Duplicate, Missing, Full, Halted, Escrow, Snap }`. All three
functions are built here, `rest_replace` included — the central catalog's
own note that it "does not exist anywhere" predates this crate's real
build and is now stale, see the thinned pointer at
[`../../../../docs/crate/015_exchange_rest.md`](../../../../docs/crate/015_exchange_rest.md).

The error surface is narrower than proposed, by design: `rest_place`
returns a plain `bool` and `rest_cancel` returns `Option<Resting>` — the
same terms [`exchange_book::Book::insert`]/[`Book::cancel`] already use —
rather than threading a `RestError` through two operations that are direct
call-throughs and add no logic of their own (see
[`../../readme.md`](../../readme.md)'s "Thin, not the proposal's full
orchestration"). Only `rest_replace`, this crate's one genuinely new
operation, gets a dedicated error type, and `RestReplaceError` carries only
the two outcomes a cancel-then-insert wrapper can itself produce —
`Missing` (nothing resting under `old_id`) and `Refused` (the replacement
was rejected by `Book::insert`, original restored) — not the proposal's
`Duplicate`, `Full`, `Halted`, `Escrow`, or `Snap`, which a thin wrapper
over `Book::insert`/`Book::cancel` cannot itself distinguish.
