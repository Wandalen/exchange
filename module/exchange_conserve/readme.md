# exchange_conserve

A batch of fills nets to zero across its signed legs, or is refused as
unbalanced.

```rust
use exact_arith::{ Price, Quantity };
use exchange_conserve::conserve_assert;
use exchange_fill::Trade;
use exchange_id::{ AccountId, OrderId };
use exchange_side::Side;

let trade = Trade
{
  taker : OrderId( 1 ), taker_account : AccountId( 1 ), taker_side : Side::Buy,
  maker : OrderId( 2 ), maker_account : AccountId( 2 ),
  price : Price::parse( "2.50" ).unwrap(), quantity : Quantity::from_int( 4 ).unwrap(),
};
assert!( conserve_assert( &[ trade ] ).is_ok() );
```

Summing the legs is `exact_arith::money_sum_assert_zero`'s job; this crate
only turns trades into signed legs.

## Not extracted from `Escrow::total_cash`/`total_asset`

The source design names those two methods as this crate's extraction source.
They check the ledger's own internal self-consistency, which is trivially
true by construction — a different check, at a different grain, from "does
this one batch of fills net to zero." See [`src/lib.rs`](src/lib.rs) for the
full reasoning, including why a single trade can never fail this check on its
own (its two legs come from one shared `price`/`quantity`, not two
independent sources) and why the batch is where this earns its keep.

## Closes hard problem 5 and feature 21

`conserve_assert` is hard problem 5 (conservation) and feature 21
(`conservation_assert`) both: one batch of fills, checked to net to exactly
zero. See [`src/lib.rs`](src/lib.rs)'s own module doc for why this is a
different, narrower check than `Escrow::total_cash`/`total_asset`.

## Wired into `exchange_match`

`exchange_match::cross_inner` calls `conserve_assert` on its own batch of
trades before returning, as defense-in-depth — `MatchError::Conservation`
wraps a refusal, matching the crate's existing, pre-established pattern for
`MatchError::Quantity`/`BookDesynchronized`: documented as unreachable
through the real submission path today, kept because a trade's two legs
coming from the same `price`/`quantity` (so they always offset) stops being
guaranteed the day a fee or similar asymmetry enters this family's design.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_fill`, `exchange_side`, `exchange_types`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `ConserveError`, `conserve_assert` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| `docs/pitfall/` | The 1 "Money" pitfall this crate's exact-arithmetic choice bears on |
| [`tests/exchange_conserve_test.rs`](tests/exchange_conserve_test.rs) | Test Matrix T01 — leg summation and batch assertion, plus Phase P17's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_fill/`](../exchange_fill/readme.md) — supplies `Trade` and its `taker_side` field
- [`exchange_escrow/`](../exchange_escrow/readme.md) — the ledger-level conservation check this crate is *not* (see above)
- [`exchange_match/`](../exchange_match/readme.md) — the real caller, via `cross_inner`
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p17_cons`, this crate's phase smoke
