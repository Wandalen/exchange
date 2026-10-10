# exchange_fill

The trade record and the event stream — the Contract's two outputs: `Trade`,
`Event`, `EventKind`, `RejectReason`, `CancelCause`. Depends on
`exchange_id`, `exchange_order`, `exchange_seq`, `exchange_side` and
`exact_arith`. Closes hard problem 21 and features 12 and 13.

```rust
use exchange_fill::Trade;
use exact_arith::Price;

let maker = Price::parse( "2.50" ).unwrap();
let generous_taker = Price::parse( "3.00" ).unwrap();
assert_eq!( Trade::executed_price( maker, generous_taker ), maker );
```

## `Trade`, not `Fill`

One record per match, naming both sides — `taker`/`taker_account`/
`taker_side` and `maker`/`maker_account` — and executing at the maker's price
(`Trade::executed_price`). `taker_side` is for `exchange_conserve`, which
classifies a batch of trades with no submission context.

## Rejects and cancels are events

The source design's `Reject` and `CancelAck` are `EventKind::OrderRejected`
and `EventKind::OrderCancelled`; the order and account travel on the
enclosing `Event`. Every `EventKind` consumer in the family names its variant
— none has a wildcard arm a new kind could fall into (manual plan M2).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_id`, `exchange_order`, `exchange_seq`, `exchange_side`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `Trade`, `Event`, `EventKind`, `RejectReason`, `CancelCause` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_fill_test.rs`](tests/exchange_fill_test.rs) | Test Matrix T01 — executed price, field shape, plus Phase P16's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_order/`](../exchange_order/readme.md) — supplies `Obligation`, which `EventKind` carries
- [`exchange_types/`](../exchange_types/readme.md) — `notional`, which prices a `Trade`
- [`exchange_conserve/`](../exchange_conserve/readme.md) — classifies a batch by `Trade::taker_side`
- [`exchange_core/`](../exchange_core/readme.md) — emits every `Event`
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p16_fill`
