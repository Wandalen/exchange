# exchange_spec

Tick, lot, the asset pair, and the halt flag on one instrument. Depends on
`exchange_id` (for `InstrumentId`) and `exact_arith` (for the decimal grid
itself) — nothing else. Closes hard problems 1 (one book per instrument), 8
(tick and lot), 18 (halt) and 19 (more than one asset), and features 4
(`InstrumentSpec`) and 5 (`price_snap`/`qty_snap`).

```rust
use exact_arith::{ Price, Quantity };
use exchange_id::InstrumentId;
use exchange_spec::{ AssetId, price_snap, spec_new };

let spec = spec_new( InstrumentId( 1 ), AssetId( 1 ), AssetId( 2 ), Price::parse( "0.05" ).unwrap(), Quantity::from_int( 1 ).unwrap() ).unwrap();
assert_eq!( price_snap( &spec, Price::parse( "1.26" ).unwrap() ).unwrap(), Price::parse( "1.25" ).unwrap() );
```

## Snap is borrowed, not reimplemented

`price_snap`/`qty_snap` call `exact_arith::price_snap_tick`/`qty_snap_lot`
with the instrument's own `Tick`/`Lot` and `exact_arith::rounding_default()`
(ties-to-even) — there is no side here to pick a direction from.
`price_fits`/`qty_fits` ask the snap whether a value stays put.

## Checked on placement

`price_fits`/`qty_fits` say whether a value is on the grid. `exchange_core`
refuses an order off it — `RejectReason::PriceOffTick`/`QuantityOffLot` —
rather than snapping it to a price or size the submitter never asked for. An
order for an unregistered instrument is still accepted unchecked.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_id` + `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `AssetId`, `InstrumentSpec`, `spec_new`, `spec_halted_is`, `price_snap`, `qty_snap`, `price_fits`, `qty_fits` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_spec_test.rs`](tests/exchange_spec_test.rs) | Test Matrix T01, plus Phases P05/P06's smoke assertions |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_id/`](../exchange_id/readme.md) — supplies `InstrumentId`
- [`exchange_halt/`](../exchange_halt/readme.md) — toggles `halted`
- [`exchange_core/`](../exchange_core/readme.md) — `Exchange::spec_register`
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p05_spec`, `demo_p06_snap`
