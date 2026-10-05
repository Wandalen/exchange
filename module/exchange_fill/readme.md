# exchange_fill

The trade record and the event stream — the Contract's two outputs.

```rust
use exchange_fill::Trade;
use exact_arith::Money;

let maker = Money::parse( "2.50" ).unwrap();
let generous_taker = Money::parse( "3.00" ).unwrap();
assert_eq!( Trade::executed_price( maker, generous_taker ), maker );
```

## Extraction

`Trade`, `Event`, `EventKind`, `RejectReason` and `CancelCause` moved here
from `exchange_types`, which re-exported all five unchanged for a time —
that re-export is retired now (2026-10-05). `CancelCause`
moved alongside `EventKind` even though the plan's own text only names the
other four — `EventKind::OrderCancelled` holds a `CancelCause`, so leaving it
behind would have forced a circular dependency back up to `exchange_types`.
See [`src/lib.rs`](src/lib.rs) for the full reasoning, including why
`notional`/`TypeError`/`obligation` stay in `exchange_types` rather than
following this cluster.

## Closes hard problem 21, and features 12 and 13

`Trade` naming both `taker`/`taker_account` and `maker`/`maker_account`
alongside `price` on every match is hard problem 21 (maker and taker on
fill) and feature 12 (`Fill`) together — `instrument` is not yet one of its
fields, since every real book is still one instrument deep (see
`exchange_core`'s own "Multi-instrument" note). `RejectReason`, a closed set
rather than a string, is feature 13 (`Reject`); the order and account a
rejection concerns travel on the enclosing `Event`, not on `RejectReason`
itself.

## `taker_side`, added to `Trade`

The real struct lacked a field recording which side the taker was on.
`exchange_escrow::settle` already takes this as a separate parameter from its
caller, so the new field is redundant there — it exists for
`exchange_conserve`, which classifies a *batch* of trades with no submission
context available at all, only the trades themselves.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_id`, `exchange_order`, `exchange_seq`, `exchange_side`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `Trade`, `Event`, `EventKind`, `RejectReason`, `CancelCause` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_fill_test.rs`](tests/exchange_fill_test.rs) | Test Matrix T01 — executed-price, field shape, plus Phase P16's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_order/`](../exchange_order/readme.md) — supplies `Obligation`, which `EventKind` carries
- [`exchange_types/`](../exchange_types/readme.md) — kept `notional`/`obligation`; its re-export of this crate's five items is retired as of 2026-10-05
- [`exchange_conserve/`](../exchange_conserve/readme.md) — consumes `Trade.taker_side` to classify a batch
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p16_fill`, this crate's phase smoke
