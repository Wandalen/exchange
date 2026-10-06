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

## Wired into `exchange_inbound`, not `exchange_core`

`exchange_core::Exchange::step_place` always mints a fresh id via
`claim_order()` before an order can rest — a duplicate can never reach it
through that path, so wiring this crate in there would be permanently dead
code. `exchange_inbound::inbound_apply` is different: it is `pub`, it never
goes through `exchange_core` at all (`step_one` runs its own, separate
pipeline), and it had a real, demonstrated gap — a duplicate id's refusal was
computed and then only checked by a `debug_assert!`, silently compiled out
in release builds (`exchange_inbound/BUG-003`). `idem_insert`/`idem_remove`
close that gap exactly where it lives.

## Generic over the key

`IdSet< K = OrderId >`: any hashable key works, `OrderId` by default. A caller
that assigns `OrderId` itself keys it by `( AccountId, ClientOrderId )`.

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

- [`exchange_id/`](../exchange_id/readme.md) — supplies `OrderId`
- [`exchange_inbound/`](../exchange_inbound/readme.md) — the real caller, via `inbound_apply`'s `seen : &mut IdSet` (see "Wired into `exchange_inbound`" above — not `exchange_rest`, which stays a thin wrapper by its own module doc's design)
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p13_idem`
