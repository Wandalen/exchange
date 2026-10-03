# exchange_id

Plain id newtypes for instruments, orders and accounts — a root of the
dependency tree, with no dependency on any other `exchange_*` crate.

```rust
use exchange_id::{ OrderId, order_from_raw, order_raw };

let id = order_from_raw( 7 );
assert_eq!( order_raw( id ), 7 );
assert_eq!( OrderId( 7 ), id );
```

## Extracted from `exchange_types`

`AccountId` and `OrderId` lived in `exchange_types` until this crate split
out; `exchange_types` now depends on this crate and re-exports both, so every
existing `use exchange_types::{ AccountId, OrderId }` still resolves.
`InstrumentId` is new here and unused until the multi-instrument book lands.

## Not built: `IdError::Zero`

The source design rejects a raw id of zero. Every field here stays `pub`, so
a fallible constructor alongside direct tuple construction would be a check
with a hole already in it. See
[`docs/decisions/001_no_id_error.md`](docs/decisions/001_no_id_error.md) for
the full reasoning.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `InstrumentId`, `OrderId`, `AccountId` and their raw conversions |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why `IdError` is not part of this build |
| `docs/pitfall/` | The 1 "Identity and cap" pitfall naming this crate's own type |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_id_test.rs`](tests/exchange_id_test.rs) | Test Matrix T01 — round-trips, plus Phase P01's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_types/`](../exchange_types/readme.md) — re-exports `AccountId`/`OrderId` for existing callers
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p01_id`, this crate's phase smoke
