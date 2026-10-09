# exchange_order

One order record — a resting order or a taker — with its instrument,
time-in-force, and the submitter's own id. Depends on `exchange_id`, `exchange_side`, `exchange_tif`, and
`exact_arith` — nothing else.

```rust
use exact_arith::{ Price, Quantity };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_side::Side;
use exchange_tif::Tif;

let order = Order
{
  id : OrderId( 1 ),
  instrument : InstrumentId( 1 ),
  account : AccountId( 1 ),
  side : Side::Buy,
  price : Price::parse( "1.25" ).unwrap(),
  quantity : Quantity::from_int( 4 ).unwrap(),
  tif : Tif::Gtc,
  client : None,
};
```

## Extracted from `exchange_types`, two fields added

The real `Order` lived in `exchange_types` with five fields, missing
`instrument` and `tif` against the source design. Both are added here; every
other field is unchanged. `exchange_types` re-exported `Order` for a time so
existing callers kept resolving — that re-export is retired now (2026-10-05)
and every call site depends on this crate directly.

## `client`, added

`client : Option< ClientOrderId >` is the submitter's own id for the order —
not in the source design. `exchange_core` refuses an account's second order
under the same one, so a retry after a lost acknowledgement cannot rest twice.

## Closes feature 3

`Order` carrying id, instrument, account, side, price, quantity, and tif in
one record is feature 3 (`Order`) — now fully held: both fields the central
doc's own Design status once listed as missing (`instrument`, `tif`) are
real fields here.

## What moved, what stayed

`Obligation` moved here alongside `Order` — the family's dependency tree never
gave it a crate of its own, and it is order-shaped. The function that computes
it, `exchange_types::obligation`, stayed in `exchange_types` along with
`notional`/`TypeError`: `exchange_types` now depends on this crate forward,
rather than the other way round.

## Not built: a constructor, quantity mutation, or `OrderError`

See [`docs/decisions/001_no_order_mutation_or_error.md`](docs/decisions/001_no_order_mutation_or_error.md)
for the full reasoning — the real design keeps `Order` an immutable plain
struct, tracks a resting remainder on a separate wrapper, and validates at
the submission boundary instead.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_id` + `exchange_side` + `exchange_tif` + `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `Order`, `Obligation`, `Amount` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why no constructor, quantity mutation, or `OrderError` |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_order_test.rs`](tests/exchange_order_test.rs) | Test Matrix T01, field-distinctness |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_id/`](../exchange_id/readme.md) — supplies `InstrumentId`/`OrderId`/`AccountId`
- [`exchange_side/`](../exchange_side/readme.md) — supplies `Side`
- [`exchange_tif/`](../exchange_tif/readme.md) — supplies `Tif`
- [`exchange_types/`](../exchange_types/readme.md) — depends on this crate forward now; kept `obligation`/`notional`, retired its re-export of `Order`/`Obligation` in 2026-10-05
- [`exchange_level/`](../exchange_level/readme.md) — holds `Order` in a FIFO queue at one price
