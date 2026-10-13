# exchange_depth

Top-N book depth, read off [`exchange_book::Book`]'s priority-ordered sides
without walking past the levels it reports. Depends on `exchange_book`,
`exchange_id`, `exchange_side` and `exact_arith`. Closes hard problem 13
(depth) and feature 17 (`depth_top`).

```rust
use exchange_book::Book;
use exchange_depth::depth_top;
use exchange_id::InstrumentId;

let book = Book::new();
let depth = depth_top( &book, InstrumentId( 1 ), 10 ).unwrap();
assert!( depth.bids.is_empty() && depth.asks.is_empty() );
```

## One row per price

`Book::side` yields a side best first, with every order at one price
adjacent. `depth_top` folds consecutive orders at one price into a
`LevelView` with their summed remainder and stops at the first price past the
`n`th, so the cost is the orders in the levels it returns, not the book.

`n == 0` is refused (`DepthError::BadN`); an `n` above the number of levels
returns fewer rows.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_book`, `exchange_id`, `exchange_side`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `LevelView`, `Depth`, `DepthError`, `depth_top` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_depth_test.rs`](tests/exchange_depth_test.rs) | Test Matrix T01 — aggregation, truncation, the `BadN` guard |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_book/`](../exchange_book/readme.md) — the book read here
- [`exchange_snap/`](../exchange_snap/readme.md) — every resting order, one row each
- [`exchange_core/`](../exchange_core/readme.md) — `Exchange::depth_get` calls `depth_top`
