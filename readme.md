# exchange

A price-time-priority limit order book and matching engine: orders in, trades
and an event stream out, with escrow reservation ahead of every match and a
zero-sum conservation check on every settlement. Workstream 002's exchange
core, being split out crate by crate to match its own source design
(`docs/crate/` records the full 23-crate target and each one's status) — one
smoke lane grades the whole path end to end, a second grades each new
contract in isolation as it lands.

```rust
use exchange_core::
{
  AccountId, Exchange, InboundCmd, InstrumentId, Money, Order, OrderId, Quantity, Resting, SelfMatchPolicy,
  Sequence, Side, Tif, inbound_flush, inbound_ring, verify,
};

let instrument = InstrumentId( 1 );
let ( seller, buyer ) = ( AccountId( 1 ), AccountId( 2 ) );
let price = Money::parse( "2.50" ).unwrap();
let four = Quantity::from_int( 4 ).unwrap();

let mut exchange = Exchange::new();
exchange.open_account( seller, Money::ZERO, Quantity::from_int( 10 ).unwrap() ).unwrap();
exchange.open_account( buyer, Money::from_int( 1000 ).unwrap(), Quantity::ZERO ).unwrap();

let mut ring = inbound_ring( 8 ).unwrap();
let mut ends = ring.ends();
let ( mut producer, mut consumer ) = ends.split();

let place = | account, side, price, quantity | InboundCmd::Place
( Resting { order : Order { id : OrderId( 0 ), instrument, account, side, price, quantity, tif : Tif::Gtc, client : None }, remaining : quantity, arrival : Sequence::ZERO } );

inbound_flush( &mut producer, [ place( seller, Side::Sell, price, four ) ] );
exchange.exchange_step( &mut consumer, SelfMatchPolicy::CancelIncoming );

inbound_flush( &mut producer, [ place( buyer, Side::Buy, price, four ) ] );
let outcomes = exchange.exchange_step( &mut consumer, SelfMatchPolicy::CancelIncoming );
let receipt = outcomes[ 0 ].clone();

assert!( matches!( receipt, exchange_core::StepOutcome::Placed( Ok( ref r ) ) if r.trades.len() == 1 ) );
assert!( verify( &exchange.postings().unwrap() ).unwrap().is_balanced() );
```

To see the whole family work end to end: `cargo run -p smoke_exchange_book` —
headless, one scenario exercising multi-level matching, every `Tif`,
halt/resume, self-trade prevention, duplicate rejection, two-ring
determinism, ring overflow, and conservation, against one fixed golden
block, then exits `ok`.

## Status

Limit orders under price-time priority are implemented, tested, and graded by
the smoke lane above. Time-in-force (`GTC`/`IOC`/`FOK`/post-only) is
implemented in `exchange_match::cross` — see that crate's own module doc for
the FOK probe-then-commit rule and the post-only refusal. Three things are
deliberately not here yet, named rather than left to be discovered as gaps:

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
| [`exchange_tif/`](module/exchange_tif/readme.md) | Time-in-force as an explicit value — GTC, IOC, FOK, post-only |
| [`exchange_stp/`](module/exchange_stp/readme.md) | The self-trade policy, as a closed set |
| [`exchange_seq/`](module/exchange_seq/readme.md) | The monotonic sequence for time priority |
| [`exchange_cap/`](module/exchange_cap/readme.md) | The limit on rests and levels per book |
| [`exchange_spec/`](module/exchange_spec/readme.md) | Tick, lot, the asset pair, and the halt flag on one instrument |
| [`exchange_stats/`](module/exchange_stats/readme.md) | Running rest/fill/reject/cancel counters |
| [`exchange_types/`](module/exchange_types/readme.md) | `Price`, `TypeError`, `notional`/`obligation` — the settlement logic; no longer re-exports anything else |
| [`exchange_order/`](module/exchange_order/readme.md) | One order record — instrument and time-in-force included, plus the `Obligation` it carries |
| [`exchange_idem/`](module/exchange_idem/readme.md) | Refuses a duplicate `OrderId` before it reaches the book |
| [`exchange_fill/`](module/exchange_fill/readme.md) | `Trade`, `Event`, `EventKind`, `RejectReason`, `CancelCause` — the event-stream vocabulary |
| [`exchange_conserve/`](module/exchange_conserve/readme.md) | A fill batch nets to zero across its legs, or is refused |
| [`exchange_level/`](module/exchange_level/readme.md) | One price level — the FIFO queue underneath the book |
| [`exchange_book/`](module/exchange_book/readme.md) | The resting book, published in price-time priority order |
| [`exchange_depth/`](module/exchange_depth/readme.md) | Top-of-book depth, N levels deep, without a full walk |
| [`exchange_halt/`](module/exchange_halt/readme.md) | An on/off switch for matching, per instrument |
| [`exchange_event/`](module/exchange_event/readme.md) | The one owned drain point for the event stream |
| [`exchange_snap/`](module/exchange_snap/readme.md) | A plain, copied snapshot of a book's resting rows |
| [`exchange_match/`](module/exchange_match/readme.md) | The crossing loop, and the executed-price rule |
| [`exchange_rest/`](module/exchange_rest/readme.md) | Rest, cancel, replace — the non-matching ways an order moves on the book |
| [`exchange_inbound/`](module/exchange_inbound/readme.md) | Many ring producers, one deterministic drain into rest-or-match |
| [`exchange_escrow/`](module/exchange_escrow/readme.md) | The reservation ledger — available/reserved partition and settlement |
| [`exchange_core/`](module/exchange_core/readme.md) | Workstream 002's facade — the ring-fed `exchange_step`'s five-step pipeline over the crates below it |
| [`smoke_exchange_book/`](module/smoke_exchange_book/readme.md) | P30 — the wall smoke, exercising every stage in one run |
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

