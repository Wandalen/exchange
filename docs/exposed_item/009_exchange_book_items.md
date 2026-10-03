# Exposed Item: exchange_book

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_book`.
- **Responsibility**: The sorted bid/ask ladders for one instrument.

**Design status**: Built as the real `exchange_book` crate (`module/exchange_book/src/lib.rs`), with a different, narrower surface than proposed:
- Real: `Resting { order, remaining, arrival }`, `Book { new, insert, cancel, side, best, consume_best, len, is_empty, iter }`. Proposed: `Book { instrument, bids, asks, seq }` with `book_new, book_instrument, book_best_bid, book_best_ask, book_level_get, book_rest_count, book_price_walk`.
- No `instrument` field on `Book` — confirms single-instrument scope (see `../crate/005_exchange_spec.md`).
- `best_bid`/`best_ask` collapse into one `best(side)` taking a `Side` parameter, rather than two separately-named functions.
- No `book_level_get` (no `Level` type exists, see `../exposed_item/008_exchange_level_items.md`) and no `book_price_walk` as a named function — `iter()` exposes the whole book in priority order instead, serving a similar "walk from best" role.
- `book_rest_count` ≈ real `len()`/`is_empty()`.
- **No `BookError` type exists at all** — unlike the proposal's implied fallible design (`UnknownInstrument`/`Halted` would need a `Result`-returning API), `insert()` returns a plain `bool` and `cancel()` returns `Option<Resting>`.
- One real behavior the proposal never named: `insert()` silently refuses (returns `false`) a resting order whose id already rests, or whose `remaining` is zero (`exchange_book/src/lib.rs:136-149`) — a narrow, crate-internal form of the idempotency guard the proposal gave its own dedicated `exchange_idem` crate (see `../crate/011_exchange_idem.md`), though it reports via a boolean rather than a `Duplicate`/`RejectReason`.

### Statement

Prompt 3 specifies a `Book` scoped to one instrument with seven free functions. The real `Book` is instrument-agnostic, has no error type, collapses several proposed functions into fewer, more general ones (`side`/`best`/`iter` parameterized by `Side` rather than per-side-named pairs), and carries its own internal duplicate-id/zero-remaining guard that the proposal assigned to a separate crate.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:579-585` | Crate `exchange_book`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
