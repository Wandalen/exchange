# exchange

A price-time-priority limit order book and matching engine: orders in, trades
and an event stream out, with escrow reservation ahead of every match and a
zero-sum conservation check on every settlement. Workstream 002's exchange
core, being split out crate by crate to match its own source design
(`docs/crate/` records the full 23-crate target and each one's status) — one
smoke lane grades the whole path end to end, a second grades each new
contract in isolation as it lands.

```rust
use exchange_core::{ AccountId, Exchange, Money, Quantity, Side, verify };

let ( seller, buyer ) = ( AccountId( 1 ), AccountId( 2 ) );
let price = Money::parse( "2.50" ).unwrap();
let four = Quantity::from_int( 4 ).unwrap();

let mut exchange = Exchange::new();
exchange.open_account( seller, Money::ZERO, Quantity::from_int( 10 ).unwrap() ).unwrap();
exchange.open_account( buyer, Money::from_int( 1000 ).unwrap(), Quantity::ZERO ).unwrap();

exchange.submit( seller, Side::Sell, price, four ).unwrap();
let receipt = exchange.submit( buyer, Side::Buy, price, four ).unwrap();

assert_eq!( receipt.trades.len(), 1 );
assert!( verify( &exchange.postings().unwrap() ).unwrap().is_balanced() );
```

To see the whole family work end to end: `cargo run -p smoke_exchange_core` —
headless, no wallets beyond the escrow stub, prints a crossing trade, a
no-cross control arm, and a cancel/release, then exits `ok`.

## Status

Limit orders under price-time priority are implemented, tested, and graded by
the smoke lane above. Time-in-force (`GTC`/`IOC`/`FOK`) is implemented in
`exchange_match::cross` — see that crate's own module doc for the FOK
probe-then-commit rule. Three things are deliberately not here yet, named
rather than left to be discovered as gaps:

| Not yet built | Current behavior |
|----------------|-------------------|
| Market orders | limit only |
| Amend | cancel exists; amend does not |
| Fees | the assessment point is fixed; no schedule is decided, nothing is charged |

Self-match prevention is **not** on that list — it is implemented, with three
configurable policies enforced before a candidate fill is ever built.

## Why Not an Existing Crate

- No candidate exposes an escrow-before-rest reservation hook
- No candidate integrates this project's own exact-arithmetic types
- No candidate integrates with this project's own ring crates
- The most complete candidate carries a heavy dependency footprint
- Replay format would need adapting to match this project's own
- No single candidate combines book, ring ingress, and exact types

Full comparison against `matchcore`, `orderbook-rs`, and `limitbook`:
[`docs/research/001_exchange_vs_open_source_alternatives.md`](docs/research/001_exchange_vs_open_source_alternatives.md).

## Responsibility Table

| Directory | Responsibility |
|-----------|----------------|
| [`exchange_id/`](module/exchange_id/readme.md) | Plain id newtypes — `InstrumentId`, `OrderId`, `AccountId` |
| [`exchange_side/`](module/exchange_side/readme.md) | Bid and ask as one closed type |
| [`exchange_tif/`](module/exchange_tif/readme.md) | Time-in-force as an explicit value — GTC, IOC, FOK |
| [`exchange_stp/`](module/exchange_stp/readme.md) | The self-trade policy, as a closed set |
| [`exchange_seq/`](module/exchange_seq/readme.md) | The monotonic sequence for time priority |
| [`exchange_cap/`](module/exchange_cap/readme.md) | The limit on rests and levels per book |
| [`exchange_spec/`](module/exchange_spec/readme.md) | Tick, lot, the asset pair, and the halt flag on one instrument |
| [`exchange_types/`](module/exchange_types/readme.md) | `notional`/`obligation` — the one piece of logic left; re-exports the rest |
| [`exchange_fill/`](module/exchange_fill/readme.md) | `Trade`, `Event`, `EventKind`, `RejectReason`, `CancelCause` — the event-stream vocabulary |
| [`exchange_conserve/`](module/exchange_conserve/readme.md) | A fill batch nets to zero across its legs, or is refused |
| [`exchange_book/`](module/exchange_book/readme.md) | The resting book, published in price-time priority order |
| [`exchange_match/`](module/exchange_match/readme.md) | The crossing loop, and the executed-price rule |
| [`exchange_rest/`](module/exchange_rest/readme.md) | Rest, cancel, replace — the non-matching ways an order moves on the book |
| [`exchange_escrow/`](module/exchange_escrow/readme.md) | The reservation ledger — available/reserved partition and settlement |
| [`exchange_core/`](module/exchange_core/readme.md) | Workstream 002's facade — the five-step submission path over the crates below it |
| [`smoke_exchange_core/`](module/smoke_exchange_core/readme.md) | Smoke lane 020 — one order across five crates, with a no-cross control arm |
| [`smoke_exchange_phases/`](module/smoke_exchange_phases/readme.md) | The P01–P29 phase-smoke ladder — one new contract graded per phase |
| [`license`](license) | MIT license text |
| [`.github/`](.github/workflows/ci.yml) | CI — nextest, doctests, clippy, and a clean rustdoc build on every push/PR |

Each crate's own readme explains its design decisions and trade-offs in
depth — this page only orients; nothing below is duplicated here.

## Build & Test

```bash
cargo check --workspace
cargo nextest run --workspace --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

The same three commands CI runs on every push and pull request — see
[`.github/workflows/ci.yml`](.github/workflows/ci.yml).

## Provenance

Extracted from the `codename_space_sandbox` monorepo into its own repository —
same pattern as [`../exact`](../exact/readme.md) and [`../ring`](../ring/readme.md) —
and depends on `exact_arith` (`../exact/module/exact_arith`) for every price,
quantity, and balance type it uses. The workstream's own instance doc,
`codename_space_sandbox/docs/workstream/002_exchange_core/readme.md`, stayed
behind in the monorepo and has no relative path that reaches it from here
anymore; look it up there directly rather than following a link.

Licensed under MIT — see [`license`](license).

