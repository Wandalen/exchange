# exchange_core

Order matching. Orders in,
trades and an event stream out, under price-time priority with escrow
reservation and event-atomic release. Every price and balance is
[`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md); no ECS type appears
anywhere in the family, in public API or in a private field.

```rust
use exchange_core::
{
  AccountId, Exchange, InboundCmd, InstrumentId, Money, Order, OrderId, Price, Quantity, Resting,
  SelfMatchPolicy, Sequence, Side, Tif, inbound_flush, inbound_ring, verify,
};

let instrument = InstrumentId( 1 );
let ( seller, buyer ) = ( AccountId( 1 ), AccountId( 2 ) );
let price = Price::parse( "2.50" ).unwrap();
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

`id`/`arrival` on the pushed order are placeholders — [`Exchange::exchange_step`] never trusts
them, assigning both itself on drain (see that method's own doc, "owns sequencing").
`client` is trusted: an account's second order under the same `ClientOrderId` is
rejected as `DuplicateClientId`, so a retry after a lost acknowledgement cannot rest twice.

Created per `task/decisions.md` Q-10.
Implemented 2026-08-30 and graded from outside by
[`smoke_exchange_book`](../smoke_exchange_book/readme.md).

## A facade over the family's crates

This crate owns `Exchange` — the five-step submission path — and re-exports
everything else. Its own logic is the sequencing:

1. **validate** — a zero quantity is `Rejected`, an unknown account is `Rejected`;
2. **reserve** — whole or not at all, *before* the book sees the order;
3. **match** — the crossing loop, taking from the front of the book;
4. **settle** — each trade, in order, releasing price improvement to the buyer;
5. **dispose** — the remainder rests; every step emits its event.

| Crate | Tier | Owns |
|-------|------|------|
| [`exchange_types`](../exchange_types/readme.md) | 0 | `Price`, `TypeError`, `notional`/`obligation` |
| [`exchange_book`](../exchange_book/readme.md) | 1 | The resting book and its published order |
| [`exchange_match`](../exchange_match/readme.md) | 2 | The crossing loop and the executed-price rule |
| [`exchange_escrow`](../exchange_escrow/readme.md) | 2 | The reservation ledger and settlement |
| `exchange_core` | 3 | This facade, and `Exchange` |
| [`smoke_exchange_book`](../smoke_exchange_book/readme.md) | lane | The wall smoke, exercising every stage in one run |

The lane depends on this crate for everything except idempotency and
conservation — two properties this facade deliberately never wires in (see
the lane's own readme) — so a re-export missing from the facade is still a
build failure over there rather than a gap that surfaces at the first
external consumer — the construction
[`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md) uses for the same reason.

This table predates most of the family's later decomposition and is kept here
for the crates it names, not as the full count — see
[`module/readme.md`](../readme.md) for every crate `exchange_core` actually
depends on today.

## Closes hard problem 4, and feature 10

Step 2 reserves before the order is visible to matching, and releases on
cancel; step 5 rests only what the match didn't consume, reservation
retained — together, this is `Exchange::exchange_step`/`Exchange::cancel`'s
own load-bearing ordering, not a restatement of `exchange_escrow`'s or
`exchange_book`'s own, narrower contracts. Closes hard problem 4
(escrow before rest) and feature 10 (partial fill, rest the leftover).

## Hosting, and what is not here

Integrated with a real concurrency host now: [`exchange_core::exchange_step`](src/lib.rs)
drains an [`exchange_inbound`](../exchange_inbound/readme.md) ring and applies
every command it finds, in drain order — see that method's own doc for the
full design. The apply loop itself stays single-threaded by choice, not by
limitation: it is the ring's producers that genuinely race, not
`exchange_step`'s own drain-then-apply pass over what they left behind.

Three things are deliberately absent, named here rather than left to be
discovered as gaps:

- **Market orders.** Limit only.
- **Amend.** [`algorithm/003`](docs/algorithm/003_cancel_and_amend_semantics.md) covers cancel, which exists; amend does not.
- **Fees.** [`algorithm/004`](docs/algorithm/004_fee_assessment_point.md) fixes the assessment point; no schedule is decided, so nothing is charged.

Two more are resolved, not absent, as a side effect of routing through the
ring rather than a direct call: **Time-in-Force** is a real, caller-chosen
field on every placed order now (`{ FOK, IOC, GTC }`, no longer pinned to
`GTC`), and **self-match policy** is a parameter on
[`Exchange::exchange_step`] itself, supplied per call rather than hardcoded.
[`algorithm/002`](docs/algorithm/002_self_match_prevention.md) states the
self-match ban;
[`exchange_match/tests/self_match_test.rs`](../exchange_match/tests/self_match_test.rs)
covers all three configured policies. `Replace` is drained but not yet
applied — see `exchange_step`'s own doc and
[`docs/decisions/`](docs/decisions/readme.md) for why.

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
| [`Cargo.toml`](Cargo.toml) | Manifest — every crate this facade depends on and re-exports |
| [`src/lib.rs`](src/lib.rs) | `Exchange`, the ring-fed `exchange_step` path, and the re-export block |
| [`docs/`](docs/readme.md) | Scope, related crates, and open trade-offs |
| [`tests/submission_test.rs`](tests/submission_test.rs) | Test Matrix T11 — the path end to end, replay, and the postings audit |
| [`tests/contract_test.rs`](tests/contract_test.rs) | Test Matrix T12 — no ECS type and no float, over the whole family's source |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan — book-insert guard, walk-completeness guard, order-id/reservation guard |
| [`task/`](task/) | Crate-scoped work items |
