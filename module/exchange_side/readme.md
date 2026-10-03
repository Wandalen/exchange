# exchange_side

Bid and ask as one closed type — a root of the dependency tree, with no
knowledge of price.

```rust
use exchange_side::Side;

assert_eq!( Side::Buy.opposite(), Side::Sell );
```

## Extracted from `exchange_types`

`Side` lived in `exchange_types` until this crate split out; `exchange_types`
now depends on this crate and re-exports it, so every existing
`use exchange_types::Side` still resolves.

## Naming — `Buy`/`Sell`, not `Bid`/`Ask`

The source design names the variants `Bid`/`Ask`. The real type keeps
`Buy`/`Sell` because the rest of the family reasons about the economic
action, not the display term — see
[`docs/decisions/001_buy_sell_not_bid_ask.md`](docs/decisions/001_buy_sell_not_bid_ask.md).
[`side_is_bid`]/[`side_is_ask`] give the source's vocabulary a home without
moving it onto the type.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `Side`, its opposite relation, and the bid/ask query helpers |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why the variants are `Buy`/`Sell`, not `Bid`/`Ask` |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_side_test.rs`](tests/exchange_side_test.rs) | Test Matrix T01 — the opposite relation, plus Phase P02's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_types/`](../exchange_types/readme.md) — re-exports `Side` for existing callers
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p02_side`, this crate's phase smoke
