# exchange_halt

Halt and resume one instrument: while halted, `exchange_core` refuses new
orders on it with `RejectReason::Halted`; resting orders and cancels are
untouched. Depends on `exchange_spec` only. Closes hard problem 18 and
feature 18.

```rust
use exact_arith::{ Price, Quantity };
use exchange_halt::{ halt_is, halt_set };
use exchange_id::InstrumentId;
use exchange_spec::{ AssetId, spec_new };

let mut spec = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();
halt_set( &mut spec ).unwrap();
assert!( halt_is( &spec ) );
```

## Wiring

`Exchange::halt_set`/`halt_clear`/`halt_is` call straight through by name.
`step_place` reads `halt_is` after the instrument check and before the grid
check. A second halt, or a resume of a running instrument, is refused with
`HaltError::Already` rather than ignored.

## No `exchange_book` dependency

The source design names one; nothing here reads a book — see
[`docs/decisions/001_no_exchange_book_dependency.md`](docs/decisions/001_no_exchange_book_dependency.md).

## Not built: auction reopen, scheduled resume

Venues usually reopen a halted market with an auction — collect orders
without matching, then cross them at one price. That needs an uncrossing step
`exchange_match` does not have, and a halted instrument here refuses new
orders rather than collecting them. A scheduled resume needs a clock, and the
family reads none.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_spec` only |
| [`src/lib.rs`](src/lib.rs) | `HaltError`, `halt_set`, `halt_clear`, `halt_is` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why `exchange_book` is not a dependency |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_halt_test.rs`](tests/exchange_halt_test.rs) | Test Matrix T01 — toggling and the `Already` guard |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_spec/`](../exchange_spec/readme.md) — owns the `halted` flag
- [`exchange_core/`](../exchange_core/readme.md) — the caller, refusing placements on a halted instrument
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p25_halt`
