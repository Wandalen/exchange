# exchange_order

One order record — a resting order or a taker — with its instrument,
time-in-force, and the submitter's own id, plus `Obligation`, what an order
commits. Depends on `exchange_id`, `exchange_side`, `exchange_tif`, and
`exact_arith` — nothing else. Closes feature 3.

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

## Fields beyond the source design

`client : Option< ClientOrderId >` is the submitter's own id for the order.
`exchange_core` refuses an account's second order under the same one, so a
retry after a lost acknowledgement cannot rest twice. The source design's
`seq` is not here — `Sequence` is stamped on `Event` and on a resting order's
`arrival`.

## Not built: a constructor, quantity mutation, or `OrderError`

`Order` is an immutable plain struct; a resting remainder lives on a separate
wrapper, and validation happens at the submission boundary — see
[`docs/decisions/001_no_order_mutation_or_error.md`](docs/decisions/001_no_order_mutation_or_error.md).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_id` + `exchange_side` + `exchange_tif` + `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `Order`, `Obligation`, `Amount` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why no constructor, quantity mutation, or `OrderError` |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_order_test.rs`](tests/exchange_order_test.rs) | Test Matrix T01 — every field reads back, values are exact |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_id/`](../exchange_id/readme.md) — supplies `InstrumentId`/`OrderId`/`AccountId`
- [`exchange_side/`](../exchange_side/readme.md) — supplies `Side`
- [`exchange_tif/`](../exchange_tif/readme.md) — supplies `Tif`
- [`exchange_types/`](../exchange_types/readme.md) — `obligation`/`notional`, computing what an `Order` commits
- [`exchange_level/`](../exchange_level/readme.md) — holds `Order` in a FIFO queue at one price
