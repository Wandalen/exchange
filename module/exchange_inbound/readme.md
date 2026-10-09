# exchange_inbound

`InboundCmd` carried from a producer, through a `ring_factory`/`ring_handle`
channel, to the book — the family's first genuinely concurrent code.

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

## Genuinely new

No ring integration exists anywhere in the real crates today — every order
reaches `exchange_core::submit` by a direct function call. This crate is the
family's first use of real concurrency, and closes hard problems 11 (inbound
at the aeon edge), 23 (full order of inbound), and 24 (ring overflow is
reject), and features 29 (inbound flush, drain, then rest or match) and 30
(overflow → reject): [`inbound_drain`] is the already-drained slice hard
problem 11 asks for; the fixed lane order the "Two producers" section below
describes is hard problem 23's stable total order; and
[`inbound_overflow_reject`]/`OverflowPolicy::Fail` is hard problem 24 and
feature 30 together. [`inbound_flush`] → [`inbound_drain`] →
[`inbound_apply`] is feature 29's flush-drain-then-rest-or-match sequence
end to end.

## Diverges from the proposal

`ring_handle::Producer` cannot be cloned — "a second producer" is refused by
that crate's own design, not a gap this crate works around. So "two
producers" is built as **two rings**, each with its own exclusively-owned
producer, genuinely raced from two OS threads, combined on the drain side in
a fixed lane order decided ahead of time rather than by which thread
finished first. See
[`docs/decisions/001_two_producers_is_two_rings.md`](docs/decisions/001_two_producers_is_two_rings.md).

`inbound_ring` always configures `OverflowPolicy::Fail` — never the ring
family's own `DropNewest` default — since a full ring's `try_push` under the
default returns `Ok` while silently discarding the record. See the module
doc's "Overflow is a reject" section.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `ring_factory`/`ring_handle`/`ring_types` plus the exchange crates `inbound_apply` dispatches to |
| [`src/lib.rs`](src/lib.rs) | `InboundCmd`, `InboundOutcome`, `InboundApplyError`, `Claims`, `inbound_ring`, `inbound_flush`, `inbound_drain`, `inbound_apply`, `inbound_overflow_reject` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why "two producers" is two rings, not one shared one |
| `docs/pitfall/` | The 3 "Ring" pitfalls this crate's ring construction and drain combine bear on |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_inbound_test.rs`](tests/exchange_inbound_test.rs) | Test Matrix T01 (ring round-trip) and T02 (`inbound_apply` dispatch) |

## Related

- [`exchange_rest/`](../exchange_rest/readme.md) — what `Cancel`/`Replace` call through to
- [`exchange_match/`](../exchange_match/readme.md) — what `Place` is crossed against before any remainder rests
- [`exchange_idem/`](../exchange_idem/readme.md) — supplies the `IdSet`s behind `Claims`: order ids checked before `Place` crosses, claimed once it rests, un-claimed on a successful `Cancel`/`Replace`; client ids checked the same way and never released
