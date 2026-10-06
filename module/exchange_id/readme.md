# exchange_id

Plain id newtypes for instruments, orders and accounts — a root of the
dependency tree, with no dependency on any other `exchange_*` crate. Closes
feature 1.

```rust
use exchange_id::{ OrderId, order_from_raw, order_raw };

let id = order_from_raw( 7 );
assert_eq!( order_raw( id ), 7 );
assert_eq!( OrderId( 7 ), id );
```

## Not built: `IdError::Zero`

The source design rejects a raw id of zero. Every field is `pub`, so a
fallible constructor next to `OrderId( 0 )` would be a check with a hole in
it — see [`docs/decisions/001_no_id_error.md`](docs/decisions/001_no_id_error.md).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `InstrumentId`, `OrderId`, `AccountId` and their raw conversions |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why there is no `IdError` |
| `docs/pitfall/` | Id reuse — what guards against it today, and where nothing does |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_id_test.rs`](tests/exchange_id_test.rs) | Test Matrix T01 — round-trips, plus Phase P01's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_book/`](../exchange_book/readme.md) — keys its storage by `InstrumentId`
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p01_id`, this crate's phase smoke
