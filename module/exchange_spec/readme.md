# exchange_spec

Tick, lot, the asset pair, and the halt flag on one instrument. Depends on
`exchange_id` (for `InstrumentId`) and `exact_arith` (for the decimal grid
itself) — nothing else.

```rust
use exact_arith::{ Price, Quantity };
use exchange_id::InstrumentId;
use exchange_spec::{ AssetId, price_snap, spec_new };

let spec = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();
assert_eq!( price_snap( &spec, Price::parse( "1.26" ).unwrap() ).unwrap(), Price::parse( "1.25" ).unwrap() );
```

## Net new

No real crate carried an instrument concept before this one — the real
`Order` struct has no `instrument` field, and nothing validates a price
against a grid. Closes hard problems 1 (one book per instrument), 8 (tick and
lot), 18 (halt), and 19 (more than one asset).

## Also closes features 4 and 5

`InstrumentSpec` itself — tick, lot, halt flag, asset pair, in one record —
is feature 4 (`InstrumentSpec`). `price_snap`/`qty_snap` are feature 5
(`price_snap`/`qty_snap`): validating a submitted price/quantity against
that grid, built on workstream 006's exact arithmetic rather than
reimplemented locally (see the next section).

## Snap is borrowed, not reimplemented

`price_snap`/`qty_snap` call `exact_arith::price_snap_tick`/`qty_snap_lot`
directly — this crate only supplies the instrument's own `Tick`/`Lot` and
picks `exact_arith::rounding_default()` (ties-to-even) over inventing a
directional default with no side to reason from. See
[`src/lib.rs`](src/lib.rs) for the full reasoning.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_id` + `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `AssetId`, `InstrumentSpec`, `spec_new`, `spec_halted_is`, `price_snap`, `qty_snap` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| [`tests/exchange_spec_test.rs`](tests/exchange_spec_test.rs) | Test Matrix T01, plus Phases P05/P06's smoke assertions |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_id/`](../exchange_id/readme.md) — supplies `InstrumentId`
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p05_spec`, `demo_p06_snap`
