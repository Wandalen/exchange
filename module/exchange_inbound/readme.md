# exchange_inbound

`InboundCmd` carried from a producer, through a `ring_factory`/`ring_handle`
channel, to the book — the family's concurrent edge.

```rust
use exchange_book::Book;
use exchange_inbound::{ inbound_apply, inbound_drain, inbound_ring, Claims, InboundCmd };
use exchange_id::{ InstrumentId, OrderId };
use exchange_stp::SelfMatchPolicy;

let mut split = inbound_ring( 8 ).unwrap();
let mut ends = split.ends();
let ( mut producer, mut consumer ) = ends.split();

producer.try_push( InboundCmd::Cancel { instrument : InstrumentId( 1 ), id : OrderId( 1 ) } ).unwrap();

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

## Related

- [`exchange_rest/`](../exchange_rest/readme.md) — what `Cancel`/`Replace` call through to
- [`exchange_match/`](../exchange_match/readme.md) — what `Place` is crossed against before any remainder rests
- [`exchange_idem/`](../exchange_idem/readme.md) — the `IdSet` behind `Claims`
- [`exchange_core/`](../exchange_core/readme.md) — drains these rings in `exchange_step`
