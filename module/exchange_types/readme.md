# exchange_types

What an order commits, priced exactly: `notional`, `obligation` and
`TypeError`. Depends on `exchange_order`, `exchange_side` and `exact_arith`.

```rust
use exchange_types::notional;
use exact_arith::{ Money, Price, Quantity };

let price = Price::parse( "2.50" ).unwrap();
let quantity = Quantity::from_int( 4 ).unwrap();
assert_eq!( notional( price, quantity ).unwrap(), Money::parse( "10" ).unwrap() );
```

## Notional is exact or it is an error

`price × quantity` at scale 6 each lands at scale 12 and has to come back to
6. The division is exact or refused: a remainder is `NotionalInexact`, never a
rounding. A rounded settlement amount leaks one minor unit at a time, in the
direction the rounding mode prefers, and every per-account sum still balances.
The product is formed in `i128`; a result that does not fit `Money` is
`NotionalOutOfRange`, not a wrap.

## A buy commits its limit

`obligation` is what an order must reserve before it rests: a sell owes the
asset, a buy owes cash at its own limit — the most it can be asked to pay.
`exchange_escrow` reserves exactly this and returns any price improvement on
settlement.

## Two prohibitions

No ECS type and no floating point, anywhere in the family.
`exchange_core/tests/contract_test.rs` walks every crate's `src/`, strips
comments, and fails on an ECS name — including a `CamelCase` one such as
`EntityId` — or on `f32`/`f64`.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exact_arith`, `exchange_order`, `exchange_side`; dev-only `exchange_id`, `exchange_tif` |
| [`src/lib.rs`](src/lib.rs) | `TypeError`, `notional`, `obligation` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface |
| [`tests/order_types_test.rs`](tests/order_types_test.rs) | Test Matrix T01 — `obligation`/`notional` exactness |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan — the inexact-notional guard |

## Related

- [`exact_arith/`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md) — `Price`, `Quantity`, `Money`
- [`exchange_order/`](../exchange_order/readme.md) — `Order`, `Obligation`
- [`exchange_escrow/`](../exchange_escrow/readme.md) — reserves an `obligation`, settles at a `notional`
- [`exchange_conserve/`](../exchange_conserve/readme.md) — sums `notional`s across a batch
