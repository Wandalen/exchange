# item

The exposed surface of `exchange_book`, as built — a consolidated index
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
| `Resting` | type alias | `= exchange_level::LevelNode` |
| `Book` | struct | `{ per_instrument: Vec<(InstrumentId, InstrumentBook)> }` — field private; `InstrumentBook` is a private helper struct, not part of the public surface |
| `Book::new` | fn | `() -> Self` |
| `Book::insert` | fn | `(&mut self, resting: Resting) -> bool` |
| `Book::cancel` | fn | `(&mut self, instrument: InstrumentId, id: OrderId) -> Option<Resting>` |
| `Book::side` | fn | `(&self, instrument: InstrumentId, side: Side) -> impl Iterator<Item = &Resting>` |
| `Book::best` | fn | `(&self, instrument: InstrumentId, side: Side) -> Option<&Resting>` |
| `Book::consume_best` | fn | `(&mut self, instrument: InstrumentId, side: Side, taken: Quantity) -> bool` |
| `Book::len` | fn | `(&self) -> usize` |
| `Book::is_empty` | fn | `(&self) -> bool` |
| `Book::iter` | fn | `(&self) -> impl Iterator<Item = &Resting>` |
| `Book::rests_at` | fn | `(&self, instrument: InstrumentId, side: Side, price: Price) -> usize` — added for `exchange_cap`'s wiring into `exchange_core` |
| `Book::level_count` | fn | `(&self, instrument: InstrumentId, side: Side) -> usize` — added for `exchange_cap`'s wiring into `exchange_core` |

### Differs from the proposal

Verified directly against `core_exchange.txt:404-408` (crate 9, Prompt 2)
and `core_exchange.txt:579-585` (exposed-item list, Prompt 3) — not against
the central `docs/crate/009_exchange_book.md`/`docs/exposed_item/009_exchange_book_items.md`
summaries, both of which described an earlier, truly single-instrument build
with no `InstrumentId` anywhere in the API; that description is now stale on
two counts (there is now instrument-keying, and it runs the opposite
direction from what the proposal named) and has been corrected in place to
point here for the current comparison, per this session's "fix the
Design-status line if stale" allowance.

The proposal specifies one `Book { instrument, bids, asks, seq }` value
*per instrument* — the caller holds the keying (a `Vec<Book>` or
`HashMap<InstrumentId, Book>` outside this crate) — plus seven free
functions (`book_new`, `book_instrument`, `book_best_bid`, `book_best_ask`,
`book_level_get`, `book_rest_count`, `book_price_walk`) and
`BookError { UnknownInstrument, Halted }`. The real build inverts the
keying direction instead of matching it field-for-field:

- **One `Book` value holds every instrument**, keyed internally by a sorted
  `Vec<(InstrumentId, InstrumentBook)>` rather than the caller holding one
  `Book` per instrument — see `src/lib.rs`'s own module doc, "One book per
  instrument," for the reasoning (`exchange_core::Exchange::cancel` needs a
  global, instrument-unaware lookup by id alone, which only works if one
  `Book` value can answer for every instrument). `cancel`/`side`/`best`/
  `consume_best` each take `instrument: InstrumentId` as their own first
  argument instead of reading it off a stored field, because (unlike
  `insert`, which reads it from `resting.order.instrument`) none of the
  other four has an order in hand to read it from.
- `best_bid`/`best_ask` collapse into one `best(instrument, side)` taking a
  `Side` parameter, same as the pre-rework build already did — the
  instrument-keying change is additive to that collapse, not a reversal of
  it.
- No `book_level_get` (no `Level` type is visible at this crate's boundary —
  see [`../../../../docs/exposed_item/008_exchange_level_items.md`](../../../../docs/exposed_item/008_exchange_level_items.md))
  and no `book_price_walk` as a named function — `iter()` exposes the whole
  book (every instrument, bids then asks, each side in priority order) in
  one pass instead.
- `book_rest_count` ≈ real `len()`/`is_empty()`.
- **No `BookError` type exists at all.** `insert()` returns a plain `bool`
  and `cancel()` returns `Option<Resting>` instead of a `Result` carrying
  `UnknownInstrument`/`Halted`. An instrument nothing has ever inserted into
  behaves exactly like an empty book — `side`/`best` return empty/`None`,
  `cancel` returns `None` — so there is no separate "unknown instrument"
  condition to report in the first place; `Halted` belongs to
  `exchange_halt`, a different crate's own responsibility (this crate's own
  Scope: "No match, no escrow, no `ring_*` import" already excludes it).
  Reasoning for the bool/`Option` shape specifically (not a dedicated error
  enum) is in `src/lib.rs` itself: `insert`'s own doc comment and its two
  `Fix(...)` comments explain exactly why a duplicate id or a zero-remaining
  `Resting` is refused rather than reported through a richer type.
- One real behavior the proposal never named: `insert()` silently refuses
  (returns `false`) a resting order whose id already rests — on *either*
  side, not just the matching one — or whose `remaining` is zero
  (`src/lib.rs`, `Book::insert`) — a narrow, crate-internal form of the
  idempotency guard the proposal gave its own dedicated `exchange_idem`
  crate (see [`../../../../docs/crate/011_exchange_idem.md`](../../../../docs/crate/011_exchange_idem.md)),
  reported via a boolean rather than a `Duplicate`/`RejectReason`.

No `docs/decisions/` collection: each divergence above is already argued at
its own site in `src/lib.rs` (module doc for the instrument-keying
inversion, `insert`'s own doc plus its `Fix(...)` comments for the
bool-return choice) — a separate ADR-style file would restate reasoning
that is already written down exactly where a future change to it would have
to be made, rather than add anything a reader can't already find there.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:404-408` | Crate 9 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:579-585` | Crate `exchange_book`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
