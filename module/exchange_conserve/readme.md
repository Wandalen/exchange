# exchange_conserve

A batch of fills nets to zero across its signed legs, or is refused as
unbalanced.

```rust
use exact_arith::Money;
use exchange_conserve::fill_legs_sum;

let legs = [ Money::from_int( 10 ).unwrap(), Money::from_int( -10 ).unwrap() ];
assert_eq!( fill_legs_sum( &legs ).unwrap(), Money::ZERO );
```

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

## Not yet wired into `exchange_match`

`conserve_assert` is built and tested standalone; calling it from
`exchange_match::cross` after each batch of trades is Stage 6's job (that
crate's own TIF-aware rework), not this crate's — same skeleton-first pattern
already used for `exchange_cap` and `exchange_level`.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_fill`, `exchange_side`, `exchange_types`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `ConserveError`, `fill_legs_sum`, `conserve_assert` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| `docs/pitfall/` | The 1 "Money" pitfall this crate's exact-arithmetic choice bears on |
| [`tests/exchange_conserve_test.rs`](tests/exchange_conserve_test.rs) | Test Matrix T01 — leg summation and batch assertion, plus Phase P17's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_fill/`](../exchange_fill/readme.md) — supplies `Trade` and its `taker_side` field
- [`exchange_escrow/`](../exchange_escrow/readme.md) — the ledger-level conservation check this crate is *not* (see above)
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p17_cons`, this crate's phase smoke
