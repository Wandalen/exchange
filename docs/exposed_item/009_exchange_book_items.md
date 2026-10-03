# Exposed Item: exchange_book

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_book`.
- **Responsibility**: The sorted bid/ask ladders for one instrument.

**Design status**: Built as the real `exchange_book` crate (`module/exchange_book/src/lib.rs`), with a different, narrower surface than proposed. Superseded note, corrected in this pass: an earlier real build was truly single-instrument (no `InstrumentId` anywhere in its API); the current build is multi-instrument instead — one `Book` value holds every instrument internally, keyed by `InstrumentId` — see [`../../module/exchange_book/docs/item/readme.md`](../../module/exchange_book/docs/item/readme.md) for the full, verified comparison. Summary:
- Real: `Resting { order, remaining, arrival }`, `Book { new, insert, cancel, side, best, consume_best, len, is_empty, iter }` — with `cancel`/`side`/`best`/`consume_best` each now taking `instrument: InstrumentId` as their own first argument. Proposed: `Book { instrument, bids, asks, seq }` with `book_new, book_instrument, book_best_bid, book_best_ask, book_level_get, book_rest_count, book_price_walk` — one `Book` *value* per instrument, keyed by the caller. The real build inverts that: one `Book` value, keyed internally.
- `best_bid`/`best_ask` collapse into one `best(instrument, side)` taking a `Side` parameter, rather than two separately-named functions.
- No `book_level_get` (no `Level` type exists, see `../exposed_item/008_exchange_level_items.md`) and no `book_price_walk` as a named function — `iter()` exposes the whole book (every instrument) in priority order instead, serving a similar "walk from best" role.
- `book_rest_count` ≈ real `len()`/`is_empty()`.
- **No `BookError` type exists at all** — unlike the proposal's implied fallible design (`UnknownInstrument`/`Halted` would need a `Result`-returning API), `insert()` returns a plain `bool` and `cancel()` returns `Option<Resting>`. An instrument nothing has ever inserted into behaves exactly like an empty book, so there is no separate "unknown instrument" condition to report.
- One real behavior the proposal never named: `insert()` silently refuses (returns `false`) a resting order whose id already rests, or whose `remaining` is zero (`module/exchange_book/src/lib.rs`, `Book::insert`) — a narrow, crate-internal form of the idempotency guard the proposal gave its own dedicated `exchange_idem` crate (see `../crate/011_exchange_idem.md`), though it reports via a boolean rather than a `Duplicate`/`RejectReason`.

### Statement

Prompt 3 specifies one `Book` scoped to one instrument, keyed externally by the caller, with seven free functions. The real `Book` holds every instrument itself (internally keyed, the opposite direction from the proposal), has no error type, collapses several proposed functions into fewer, more general ones (`side`/`best`/`iter` parameterized by `Side` rather than per-side-named pairs), and carries its own internal duplicate-id/zero-remaining guard that the proposal assigned to a separate crate.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:579-585` | Crate `exchange_book`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
