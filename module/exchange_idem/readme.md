# exchange_idem

`OrderId` seen once per book — a retry that resubmits the same id is rejected
rather than doubling the rest. Depends on `exchange_id` only.

```rust
use exchange_id::OrderId;
use exchange_idem::{ IdSet, idem_insert, idem_seen };

let mut seen = IdSet::new();
idem_insert( &mut seen, OrderId( 1 ) ).unwrap();
assert!( idem_seen( &seen, OrderId( 1 ) ) );
assert!( idem_insert( &mut seen, OrderId( 1 ) ).is_err(), "a repeat is refused" );
```

## Net new

No real crate carried idempotency before this one — nothing tracked which
`OrderId`s had already been accepted, so a caller that retried a submission
after a dropped acknowledgement could double-rest the same order. Closes hard
problem 15 (idempotent order ids) and feature 19 (unique `OrderId` per book)
— `exchange_book::Book::insert` already refused a repeat silently; this
crate gives that one case its own checkable, named error, pre-emptively.

## Two callers, two keys

- `exchange_inbound::inbound_apply` keys by `OrderId`: it is `pub`, bypasses
  `exchange_core`, and lets its caller pick ids, so a repeated `OrderId` is a
  real retry there. It keys a second set by `( AccountId, ClientOrderId )`,
  as `exchange_core` does.
- `exchange_core` keys by `( AccountId, ClientOrderId )`: it assigns
  `OrderId` itself, so only the submitter's own id can mark a retry.

## Generic over the key

`IdSet< K = OrderId >`: any hashable key works, `OrderId` by default.

## `idem_remove` exists so a cancelled id can be resubmitted

Without it, a legitimate cancel-then-resubmit under the same id would be
indistinguishable from the retry this crate exists to refuse.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_id` only |
| [`src/lib.rs`](src/lib.rs) | `IdSet`, `idem_seen`, `idem_insert`, `idem_remove`, `IdemError` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | The 1 "Identity and cap" pitfall this crate's refusal behavior avoids |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing — matches the proposal exactly |
| [`tests/exchange_idem_test.rs`](tests/exchange_idem_test.rs) | Test Matrix T13, plus Phase P13's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_id/`](../exchange_id/readme.md) — supplies `OrderId`, `AccountId`, `ClientOrderId`
- [`exchange_inbound/`](../exchange_inbound/readme.md) — caller keyed by `OrderId` and by `( AccountId, ClientOrderId )`, via `inbound_apply`'s `Claims`
- [`exchange_core/`](../exchange_core/readme.md) — caller keyed by `( AccountId, ClientOrderId )`
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p13_idem`
