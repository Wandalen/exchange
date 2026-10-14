# exchange_inbound

`InboundCmd` carried from a producer, through a `ring_factory`/`ring_handle`
channel, to the book — the family's concurrent edge.

```rust
use exchange_book::Book;
use exchange_inbound::{ inbound_apply, inbound_drain, inbound_ring, Claims, InboundCmd, OrderRef };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_stp::SelfMatchPolicy;

let mut split = inbound_ring( 8 ).unwrap();
let mut ends = split.ends();
let ( mut producer, mut consumer ) = ends.split();

producer.try_push( InboundCmd::Cancel { instrument : InstrumentId( 1 ), account : AccountId( 1 ), order : OrderRef::Id( OrderId( 1 ) ) } ).unwrap();

let mut book = Book::new();
let mut claims = Claims::new();
for cmd in inbound_drain( &mut consumer )
{
  inbound_apply( &mut book, &mut claims, SelfMatchPolicy::CancelResting, cmd ).unwrap();
}
```

## Who calls it

`exchange_core::Exchange::exchange_step` drains rings built here and
applies each command itself, with escrow, caps and halt; it answers
`Replace` with `StepOutcome::ReplaceNotWired`. `inbound_apply` is the
book-level apply, without any of that: crossed, rested, cancelled and
replaced through `exchange_match` and `exchange_rest`. It refuses an order
id that rests, asking the book, and a client id its account already used,
kept in `Claims`. `smoke_exchange_phases`' P28 and P29 drive the ring.

Closes hard problems 11 (inbound at the aeon edge), 23 (full order of
inbound) and 24 (ring overflow is reject), and features 29 (flush, drain,
then rest or match) and 30 (overflow is reject).

## One producer per ring

`ring_handle::Producer` cannot be cloned, so "two producers" is two rings,
each raced from its own thread and drained in a lane order fixed ahead of
time, never by which thread finished first. The order repeats only if each
lane is drained after its producer stops; a drain of a live lane takes
whatever was published by then. See
[`docs/decisions/001_two_producers_is_two_rings.md`](docs/decisions/001_two_producers_is_two_rings.md).

`inbound_ring` always sets `OverflowPolicy::Fail`. Under the ring family's
default, `DropNewest`, a full ring's `try_push` returns `Ok` and discards
the record.

## Not built: instrument rules

`inbound_apply` refuses what would break the book or its claims — a zero
quantity, a `remaining` that is not the quantity, a resting id, a used
client id. It does not check a price's sign, tick or lot, or a halt: those
come from an instrument's `InstrumentSpec`, which `Exchange` holds, and
`exchange_step` checks them. Checking them here needs the specs passed in,
a second copy of the facade's validation.

## Not built: sequencing

A `Place` rests at the arrival its producer stamped. Lanes stamped apart can
rank an order applied later ahead of one applied before it at the same
price. `exchange_step` stamps arrivals itself on drain; doing that here
needs a counter kept between calls, and the producers of `inbound_apply`'s
callers stamp their own today.

## Not built: remembering order ids that filled

An order id is taken while its order rests. Once the order fills or leaves,
the id is free, so a retry of a filled order carrying no client id trades
again; with a client id it is refused. Keeping every order id ever placed
grows without bound and would refuse the resubmission a cancel allows.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `ring_factory`/`ring_handle`/`ring_types`, `exact_arith`, and the exchange crates `inbound_apply` calls |
| [`src/lib.rs`](src/lib.rs) | `InboundCmd`, `InboundOutcome`, `InboundApplyError`, `Claims`, `inbound_ring`, `inbound_flush`, `inbound_drain`, `inbound_apply`, `inbound_overflow_reject` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why "two producers" is two rings, not one shared one |
| `docs/pitfall/` | The three ring pitfalls: drain order, silent overflow, a shared single-producer ring |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_inbound_test.rs`](tests/exchange_inbound_test.rs) | T01, the ring round trip; T02, `inbound_apply` and its claims |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan — the overflow policy and the book check, by mutation |

## Related

- [`exchange_rest/`](../exchange_rest/readme.md) — what `Cancel`/`Replace` call through to
- [`exchange_match/`](../exchange_match/readme.md) — what `Place` is crossed against before any remainder rests
- [`exchange_idem/`](../exchange_idem/readme.md) — the `IdSet` behind `Claims`
- [`exchange_core/`](../exchange_core/readme.md) — drains these rings in `exchange_step`
