# exchange_depth

Top-N book depth, read straight off [`exchange_book::Book`]'s own
priority-ordered slices — no full walk. Depends on `exchange_book`,
`exchange_id`, `exchange_side`, and `exact_arith`.

```rust
use exchange_book::Book;
use exchange_depth::depth_top;
use exchange_id::InstrumentId;

let book = Book::new();
let depth = depth_top( &book, InstrumentId( 1 ), 10 ).unwrap();
assert!( depth.bids.is_empty() && depth.asks.is_empty() );
```

## Genuinely new

No bounded depth query exists anywhere in the real crates today —
`Book::iter()`/`Book::side()` expose a whole side unbounded; a caller
wanting "just the top few" has to walk and truncate by hand. This crate
closes hard problem 13 (depth) and feature 17 (`depth_top`), matching the
proposal's own `LevelView`/`Depth`/`depth_top`/`DepthError{BadN}` shape
exactly — no divergence to record.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exchange_book`, `exchange_id`, `exchange_side`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `LevelView`, `Depth`, `DepthError`, `depth_top` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_depth_test.rs`](tests/exchange_depth_test.rs) | Test Matrix T01 — aggregation, truncation, the `BadN` guard |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |
