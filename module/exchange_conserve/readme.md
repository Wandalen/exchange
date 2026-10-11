# exchange_conserve

A batch of fills nets to zero across its legs, or is refused. Closes hard
problem 5 and feature 21.

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
only turns trades into legs — the buyer debited the notional, the seller
credited it.

## Unbalanced is unreachable today

A trade's two legs come from one `price`/`quantity`, so they always cancel.
`exchange_match::cross` calls `conserve_assert` on every batch anyway, and
wraps a refusal in `MatchError::Conservation`: the day a fee makes a debit
and its credit differ, this is the check that catches a fee miscomputed on
one side. What can fail today is a trade whose notional is not exact — see
[`src/lib.rs`](src/lib.rs).

## Not `Escrow::total_cash`/`total_asset`

The source design names those as this crate's extraction source. They sum the
ledger's own balances, which agree by construction; this checks one batch of
trades, before escrow or the event stream sees it.

## Not built: fee legs

A fee turns a trade's two legs into three — buyer, seller, and the fee sink —
and makes `Unbalanced` reachable. The point of assessment is decided
(`exchange_core/docs/algorithm/004_fee_assessment_point.md`); the schedule is
not, and the sink is workstream 010's
([`../../docs/not_a_002_crate/004_wallet_fee_sink.md`](../../docs/not_a_002_crate/004_wallet_fee_sink.md)).

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_fill`, `exchange_side`, `exchange_types`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `ConserveError`, `conserve_assert` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| `docs/pitfall/` | Conservation summed in floats |
| [`tests/exchange_conserve_test.rs`](tests/exchange_conserve_test.rs) | Test Matrix T01 — the batch assertion, plus Phase P17's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_fill/`](../exchange_fill/readme.md) — supplies `Trade` and its `taker_side` field
- [`exchange_escrow/`](../exchange_escrow/readme.md) — the ledger-level totals this crate is not
- [`exchange_match/`](../exchange_match/readme.md) — calls it from `cross`
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p17_cons`, this crate's phase smoke
