# exchange_snap

Plain, copied rows of what rests on a book — independent of the live `Book`
the moment they're taken. Depends on `exchange_book` and `exchange_id`, plus
the family's own `exact_arith` facade.

```rust
use exact_arith::Money;
use exchange_book::Book;
use exchange_id::InstrumentId;
use exchange_snap::{ snap_len, snap_take };

let book = Book::new();
let snap = snap_take( &book, InstrumentId( 1 ), Money::parse( "0.01" ).unwrap() );
assert_eq!( snap_len( &snap ), 0 );
```

## Genuinely new

No snapshot type exists anywhere in the real crates today —
`exchange_book::Book::iter()` can walk the live book, but nothing can be
persisted, compared, or diffed without reaching back into it. This crate
closes hard problems 16 (closed types) and 22 (snapshot of a book), and
feature 27 (book snapshot rows).

## Diverges from the proposal

`RestRow`/`BookSnap`'s fields match the proposal exactly — see
[`docs/item/readme.md`](docs/item/readme.md) for the field-by-field check.
Two real divergences get their own ADRs: no `exchange_order` dependency
([`docs/decisions/001_no_exchange_order_dependency.md`](docs/decisions/001_no_exchange_order_dependency.md))
and no `SnapError { Full }`
([`docs/decisions/002_no_snap_error.md`](docs/decisions/002_no_snap_error.md)).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exchange_book`, `exchange_id`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `RestRow`, `BookSnap`, `snap_take`, `snap_len` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why `exchange_order` and `SnapError` are not part of this build |
| `docs/pitfall/` | The 3 snapshot pitfalls this crate avoids by construction |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_snap_test.rs`](tests/exchange_snap_test.rs) | Test Matrix T01 — content, ordering, independence from the live book |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual mutation-testing plan |
