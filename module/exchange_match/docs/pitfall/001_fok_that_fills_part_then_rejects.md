# Pitfall: FOK that fills part, then rejects

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Letting a Fill-or-Kill order partially execute before the rejection is decided.
- **In Scope**: `cross`'s FOK handling via `tif_requires_full`.

### Statement

FOK is all or nothing, so the book must be checked for a full fill before any
quantity changes hands. Once part of the order has traded, rejecting it can
no longer leave the book as it was.

### How this crate avoids it

`cross` ([`src/lib.rs`](../../src/lib.rs)) runs the crossing loop against a
clone of the book first, and runs it on the real book only if that probe
consumed the whole order. Otherwise it returns a no-cross — no trades, the
full quantity remaining, nothing cancelled — with the real book untouched.
Turning that into a rejection is `exchange_core`'s job; committing a partial
FOK fill has no path here.

[`tests/tif_test.rs`](../../tests/tif_test.rs)'s
`fok_that_cannot_fill_entirely_leaves_the_book_untouched` and
`fok_short_by_a_second_level_rejects_the_whole_order_and_restores_the_first`
build a book the FOK cannot fill and check the resting orders afterwards.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/016_fok_that_fills_part_then_rejects.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:913` | Pitfall in the source's Prompt 7 "Match policy" list |
