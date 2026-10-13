# exchange_snap

Plain, copied rows of what rests on one instrument's book, independent of the
live `Book` once taken. Depends on `exchange_book`, `exchange_id`,
`exchange_side` and `exact_arith`. Closes hard problems 16 (closed types) and
22 (snapshot of a book), and feature 27 (book snapshot rows).

```rust
use exchange_book::Book;
use exchange_id::InstrumentId;
use exchange_snap::{ snap_len, snap_take };

let book = Book::new();
let snap = snap_take( &book, InstrumentId( 1 ), 0 );
assert_eq!( snap_len( &snap ), 0 );
```

## Rows, not a save format

`BookSnap` holds one row per resting order — id, side, price, remainder —
copied out of the book, bids then asks, each side in priority order. A later
cancel or fill cannot reach it, and two snapshots compare with `==`. It is the
row shape another crate persists or diffs — not file format 012, which stores
the rows.

`tick` is the caller's step count, stamped as given: the simulation's clock,
not the exchange's, and never `Instant::now()`.

## Diverges from the proposal

No `exchange_order` dependency
([`docs/decisions/001_no_exchange_order_dependency.md`](docs/decisions/001_no_exchange_order_dependency.md))
and no `SnapError { Full }`
([`docs/decisions/002_no_snap_error.md`](docs/decisions/002_no_snap_error.md)).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_book`, `exchange_id`, `exchange_side`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `RestRow`, `BookSnap`, `snap_take`, `snap_len` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why `exchange_order` and `SnapError` are not part of this build |
| `docs/pitfall/` | The 3 snapshot pitfalls this crate avoids by construction |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_snap_test.rs`](tests/exchange_snap_test.rs) | Test Matrix T01 — content, ordering, independence from the live book |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual mutation-testing plan |

## Related

- [`exchange_book/`](../exchange_book/readme.md) — the book copied here
- [`exchange_depth/`](../exchange_depth/readme.md) — the same book, aggregated by price
- [`exchange_core/`](../exchange_core/readme.md) — `Exchange::snap_take` calls `snap_take`
