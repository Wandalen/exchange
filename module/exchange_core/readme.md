# exchange_core

Order matching. Orders in,
trades and an event stream out, under price-time priority with escrow
reservation and event-atomic release. Every price and balance is
[`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md); no ECS type appears
anywhere in the family, in public API or in a private field.

```rust
use exchange_core::{ AccountId, Exchange, Money, Quantity, Side, verify };

let ( seller, buyer ) = ( AccountId( 1 ), AccountId( 2 ) );
let price = Money::parse( "2.50" ).unwrap();
let four = Quantity::from_int( 4 ).unwrap();

let mut exchange = Exchange::new();
exchange.open_account( seller, Money::ZERO, Quantity::from_int( 10 ).unwrap() );
exchange.open_account( buyer, Money::from_int( 1000 ).unwrap(), Quantity::ZERO );

exchange.submit( seller, Side::Sell, price, four ).unwrap();
let receipt = exchange.submit( buyer, Side::Buy, price, four ).unwrap();

assert_eq!( receipt.trades.len(), 1 );
assert!( verify( &exchange.postings().unwrap() ).unwrap().is_balanced() );
```

Created per `task/decisions.md` Q-10.
Implemented 2026-08-30 and graded from outside by
[`smoke_exchange_core`](../smoke_exchange_core/readme.md).

## A facade over four crates

This crate owns `Exchange` — the five-step submission path — and re-exports
everything else. Its own logic is the sequencing:

1. **validate** — a zero quantity is `Rejected`, an unknown account is `Rejected`;
2. **reserve** — whole or not at all, *before* the book sees the order;
3. **match** — the crossing loop, taking from the front of the book;
4. **settle** — each trade, in order, releasing price improvement to the buyer;
5. **dispose** — the remainder rests; every step emits its event.

| Crate | Tier | Owns |
|-------|------|------|
| [`exchange_types`](../exchange_types/readme.md) | 0 | Orders, trades, obligations, events, `notional` |
| [`exchange_book`](../exchange_book/readme.md) | 1 | The resting book and its published order |
| [`exchange_match`](../exchange_match/readme.md) | 2 | The crossing loop and the executed-price rule |
| [`exchange_escrow`](../exchange_escrow/readme.md) | 2 | The reservation ledger and settlement |
| `exchange_core` | 3 | This facade, and `Exchange` |
| [`smoke_exchange_core`](../smoke_exchange_core/readme.md) | lane | Smoke lane, with the no-cross control arm |

The lane depends on this crate alone, so a re-export missing from the facade is
a build failure over there rather than a gap that surfaces at the first external
consumer — the construction
[`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md) uses for the same reason.

## Closes hard problem 4, and feature 10

Step 2 reserves before the order is visible to matching, and releases on
cancel; step 5 rests only what the match didn't consume, reservation
retained — together, this is `Exchange::submit`/`Exchange::cancel`'s own
load-bearing ordering, not a restatement of `exchange_escrow`'s or
`exchange_book`'s own, narrower contracts. Closes hard problem 4
(escrow before rest) and feature 10 (partial fill, rest the leftover).

## Hosting, and what is not here

Not yet integrated with any concurrency host — the sequence is a single-thread
counter today, and nothing in the five crates reaches for one. What the design
actually *requires* of the matching path — no clock, no hash iteration order,
no address — is met by construction today, so wiring in a concurrency
mechanism later, once concurrent intake actually arrives, is additive rather
than a rewrite.

Four things are deliberately absent, named here rather than left to be
discovered as gaps:

- **Time-in-Force.** `{ FOK, IOC, GTC }` is specified; every order behaves GTC.
- **Market orders.** Limit only.
- **Amend.** [`algorithm/003`](docs/algorithm/003_cancel_and_amend_semantics.md) covers cancel, which exists; amend does not.
- **Fees.** [`algorithm/004`](docs/algorithm/004_fee_assessment_point.md) fixes the assessment point; no schedule is decided, so nothing is charged.

Self-match prevention is not on this list — it is implemented.
[`algorithm/002`](docs/algorithm/002_self_match_prevention.md) states the ban;
`Exchange::submit` enforces it via `SelfMatchPolicy::CancelIncoming`, and
[`exchange_match/tests/self_match_test.rs`](../exchange_match/tests/self_match_test.rs)
covers all three configured policies.

## Two questions this implementation closed

[`algorithm/001`](docs/algorithm/001_price_time_priority_matching.md) left both
open, and code cannot be written without answering them:

| Question | Answer | Why |
|----------|--------|-----|
| Executed price | The maker's | It is the only price both parties saw before the trade existed. The taker's limit hands the spread to whoever arrived second — a venue fee no schedule accounts for |
| Book representation | A sorted `Vec` per side | The observable property is the published order, and a flat vector *is* that order. A price-level map wins on insertion cost at depths nothing here has measured |

Both are recorded in the algorithm instance with their reasoning, and both are
reversible behind the same interfaces.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — the four crates it re-exports |
| [`src/lib.rs`](src/lib.rs) | `Exchange`, the five-step path, and the re-export block |
| [`docs/`](docs/readme.md) | Scope, related crates, and open trade-offs |
| [`tests/submission_test.rs`](tests/submission_test.rs) | Test Matrix T11 — the path end to end, replay, and the postings audit |
| [`tests/contract_test.rs`](tests/contract_test.rs) | Test Matrix T12 — no ECS type and no float, over the whole family's source |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan — book-insert guard, walk-completeness guard, order-id/reservation guard |
| [`task/`](task/) | Crate-scoped work items |
