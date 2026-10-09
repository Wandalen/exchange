# exchange_idem

`OrderId` seen once per book — a retry that resubmits the same id is rejected
rather than doubling the rest. Depends on `exchange_id` only. Closes hard
problem 15 and feature 19.

```rust
use exchange_id::OrderId;
use exchange_idem::{ IdSet, idem_insert, idem_seen };

let mut seen = IdSet::new();
idem_insert( &mut seen, OrderId( 1 ) ).unwrap();
assert!( idem_seen( &seen, OrderId( 1 ) ) );
assert!( idem_insert( &mut seen, OrderId( 1 ) ).is_err(), "a repeat is refused" );
```

## Two callers, two keys

`IdSet< K = OrderId >` takes any hashable key.

- `exchange_inbound::inbound_apply` keys by `OrderId`: it is `pub`, bypasses
  `exchange_core`, and lets its caller pick ids, so a repeated `OrderId` is a
  real retry there. It calls `idem_remove` on cancel, so a cancelled id can be
  resubmitted.
- `exchange_core` keys by `( AccountId, ClientOrderId )`: it assigns
  `OrderId` itself, so only the submitter's own id can mark a retry. It never
  removes one — the set grows with every client-tagged order.

## Not built: forgetting old client ids

Releasing a client id when its order fills or cancels would let a late retry
of that order through as a new one. Bounding the set by age needs a clock or a
retention policy, and the family reads no clock.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_id` only |
| [`src/lib.rs`](src/lib.rs) | `IdSet`, `idem_seen`, `idem_insert`, `idem_remove`, `IdemError` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | A retry resting a second order |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_idem_test.rs`](tests/exchange_idem_test.rs) | Test Matrix T13, plus Phase P13's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_id/`](../exchange_id/readme.md) — supplies `OrderId`, `AccountId`, `ClientOrderId`
- [`exchange_inbound/`](../exchange_inbound/readme.md) — caller keyed by `OrderId`, via `inbound_apply`'s `seen : &mut IdSet`
- [`exchange_core/`](../exchange_core/readme.md) — caller keyed by `( AccountId, ClientOrderId )`
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p13_dup`
