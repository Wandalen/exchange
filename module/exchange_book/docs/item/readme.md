# item

The exposed surface of `exchange_book`, as built.

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a short note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Per-declaration files.

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `Resting` | type alias | `= exchange_level::LevelNode` |
| `Book` | struct | fields private; equality compares resting orders only |
| `Book::new` | fn | `() -> Self` |
| `Book::insert` | fn | `(&mut self, resting: Resting) -> bool` |
| `Book::insertable` | fn | `(&self, resting: &Resting, vacating: Option<OrderId>) -> bool` — `insert`'s checks, nothing moved; for `exchange_rest::rest_replace` |
| `Book::cancel` | fn | `(&mut self, instrument: InstrumentId, id: OrderId) -> Option<Resting>` |
| `Book::side` | fn | `(&self, instrument: InstrumentId, side: Side) -> impl Iterator<Item = &Resting>` |
| `Book::best` | fn | `(&self, instrument: InstrumentId, side: Side) -> Option<&Resting>` |
| `Book::consume_best` | fn | `(&mut self, instrument: InstrumentId, side: Side, taken: Quantity) -> bool` |
| `Book::len` | fn | `(&self) -> usize` |
| `Book::is_empty` | fn | `(&self) -> bool` |
| `Book::iter` | fn | `(&self) -> impl Iterator<Item = &Resting>` |
| `Book::rests_at` | fn | `(&self, instrument: InstrumentId, side: Side, price: Price) -> usize` — for `exchange_cap::cap_check_rest` |
| `Book::level_count` | fn | `(&self, instrument: InstrumentId, side: Side) -> usize` — for `exchange_cap::cap_check_level` |
| `Book::account_rests` | fn | `(&self, instrument: InstrumentId, account: AccountId) -> usize` — for `exchange_cap::cap_check_account` |

### Differs from the proposal

The source design (`core_exchange.txt:404-408`, `579-585`) specifies one
`Book { instrument, bids, asks, seq }` value per instrument, keyed by the
caller, with `book_new`, `book_instrument`, `book_best_bid`, `book_best_ask`,
`book_level_get`, `book_rest_count`, `book_price_walk` and
`BookError { UnknownInstrument, Halted }`.

- **One `Book` holds every instrument**, keyed inside — `exchange_core`'s cancel
  looks an order up by id alone. Per-instrument methods take the
  `InstrumentId`; `insert` reads it off the order.
- **`best( instrument, side )`** replaces `book_best_bid`/`book_best_ask`;
  `side()` and `iter()` replace `book_price_walk`; `len()`/`is_empty()`
  replace `book_rest_count`. No `book_level_get`: no `Level` crosses this
  crate's boundary.
- **No `BookError`.** An instrument nothing rested on is an empty book, not
  an unknown one; a halt is `exchange_halt`'s. `insert` returns `false` for a
  duplicate id, or a remainder of zero or above the order's quantity, and
  `cancel` returns `None` for an absent order — see `src/lib.rs`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:404-408` | Crate 9 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:579-585` | Crate `exchange_book`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
