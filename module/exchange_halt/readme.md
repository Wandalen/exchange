# exchange_halt

An on/off switch for matching alone — resting orders are never touched.
Depends on `exchange_spec` only.

```rust
use exact_arith::{ Price, Quantity };
use exchange_halt::{ halt_is, halt_set };
use exchange_id::InstrumentId;
use exchange_spec::{ AssetId, spec_new };

let mut spec = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();
halt_set( &mut spec ).unwrap();
assert!( halt_is( &spec ) );
```

## Genuinely new

No halt/resume mechanism exists anywhere in the real crates today —
`exchange_core::Exchange::submit` always matches. This crate closes hard
problem 18 (halt) and feature 18 (`book_halt`/`book_resume`).

## Diverges from the proposal

The source design names `exchange_book` as a second dependency. Nothing in
the real match loop checks halt yet, so there is no real behavior today a
book dependency would support; see
[`docs/decisions/001_no_exchange_book_dependency.md`](docs/decisions/001_no_exchange_book_dependency.md).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — depends on `exchange_spec` only |
| [`src/lib.rs`](src/lib.rs) | `HaltError`, `halt_set`, `halt_clear`, `halt_is` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why `exchange_book` is not a dependency |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_halt_test.rs`](tests/exchange_halt_test.rs) | Test Matrix T01 — toggling and the `Already` guard |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |
