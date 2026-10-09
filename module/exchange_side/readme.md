# exchange_side

Bid and ask as one closed type — a root of the dependency tree, with no
knowledge of price. Closes feature 2.

```rust
use exchange_side::Side;

assert_eq!( Side::Buy.opposite(), Side::Sell );
```

## Naming — `Buy`/`Sell`, not `Bid`/`Ask`

The source design names the variants `Bid`/`Ask`. The family reasons about
the economic action, not the display term, so the type keeps `Buy`/`Sell`;
`side_is_bid`/`side_is_ask` cover the source's vocabulary. See
[`docs/decisions/001_buy_sell_not_bid_ask.md`](docs/decisions/001_buy_sell_not_bid_ask.md).

## Price rules

`side_ahead` (which price ranks first on a side's book) and `side_accepts`
(whether a limit accepts a price) are generic over the price type, so the
crate still knows nothing about prices. `exchange_book` and `exchange_match`
call them instead of restating the rules.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `Side`, its opposite relation, the bid/ask query helpers, and the side-dependent price rules |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why the variants are `Buy`/`Sell`, not `Bid`/`Ask` |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_side_test.rs`](tests/exchange_side_test.rs) | Test Matrix T01 — the opposite relation and price rules, plus Phase P02's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_book/`](../exchange_book/readme.md) — ranks levels with `side_ahead`
- [`exchange_match/`](../exchange_match/readme.md) — checks a limit with `side_accepts`
- [`exchange_types/`](../exchange_types/readme.md) — `obligation` branches on `Side`: a buy owes cash, a sell owes the asset
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p02_side`, this crate's phase smoke
