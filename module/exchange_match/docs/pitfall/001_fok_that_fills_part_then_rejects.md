# Pitfall: FOK that fills part, then rejects

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Letting a Fill-or-Kill order partially execute before the rejection is decided.
- **In Scope**: `cross`'s own FOK handling via `tif_requires_full`.

### Statement

FOK means all-or-nothing by definition — the book must be checked for full
fillability *before* any quantity changes hands, because once part of the
order has already traded, rejecting it can no longer leave the book
unchanged.

### How this crate avoids it

`tif_requires_full` (from `exchange_tif`) gates a FOK order into a
probe-then-commit sequence ([`src/lib.rs`](../../src/lib.rs)): the crossing
loop runs first against a disposable clone of `book`; only if that probe
fully consumes the order does the identical, deterministic sequence run
again against the real `book`. An unfillable FOK comes back shaped exactly
like an ordinary no-cross outcome — empty trades, full remaining, nothing
cancelled — because nothing in the real book ever changed. Whether to turn
"FOK, nothing filled" into an actual rejection is `exchange_core`'s decision
(this crate only reports the outcome), but the partial-execution-then-undo
failure mode itself is structurally impossible here — there is no path that
commits a partial FOK fill to the real book. Verified directly:
[`tests/tif_test.rs`](../../tests/tif_test.rs)'s
`fok_that_cannot_fill_entirely_leaves_the_book_untouched` and
`fok_short_by_a_second_level_rejects_the_whole_order_and_restores_the_first`
construct a book an incoming FOK cannot fully fill and assert the book is
byte-for-byte unchanged afterward.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/016_fok_that_fills_part_then_rejects.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:913` | Pitfall in the source's Prompt 7 "Match policy" list |
