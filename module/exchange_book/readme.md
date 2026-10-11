# exchange_book

The resting order book — per instrument, two sides, each in published order,
best at the front. Depends on `exchange_id`, `exchange_level`, `exchange_side`
and `exact_arith`. Closes hard problem 1 (one book per instrument) and
features 6 (book) and 22 (sorted price walk).

```rust
use exact_arith::{ Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

const INSTRUMENT : InstrumentId = InstrumentId( 1 );

fn bid( id : u64, arrival : u64 ) -> Resting
{
  let quantity = Quantity::from_int( 5 ).unwrap();
  Resting
  {
    order : Order
    {
      id : OrderId( id ),
      instrument : INSTRUMENT,
      account : AccountId( id ),
      side : Side::Buy,
      price : Price::parse( "2.50" ).unwrap(),
      quantity,
      tif : Tif::Gtc,
      client : None,
    },
    remaining : quantity,
    arrival : Sequence( arrival ),
  }
}

let mut book = Book::new();
// insert two bids at the same price, newest first …
book.insert( bid( 2, 20 ) );
book.insert( bid( 1, 10 ) );
// … and the earlier arrival is still at the front.
assert_eq!( book.side( INSTRUMENT, Side::Buy ).next().unwrap().order.id, OrderId( 1 ) );
```

## The contract is the order, not the storage

Matching takes from the front and never re-decides. That makes the *published
order* the whole of what this crate promises: a book holding the right orders in
the wrong sequence fills the wrong people, and nothing downstream can notice —
every trade is well-formed, every balance conserves, and the wrong account got
paid.

So every test in [`tests/priority_test.rs`](tests/priority_test.rs) reads
`side()` rather than any internal, and the ranking rule lives in one function,
`exchange_side::side_ahead`, that `insert` calls and the tests re-derive:

- better price first — higher for bids, lower for asks;
- at one price, earlier arrival first; equal arrivals in insertion order.

Price outranks arrival. The two rules only disagree when a later order is priced
better, and an implementation sorting by `( arrival, price )` passes both
individual cases and fails that one, which is why it has a test of its own.

## Arrival is a claimed number, never a clock

`arrival` is a `Sequence` handed in by the caller. The arrival-order invariant
this crate must uphold forbids the matching path from reading a clock, and a
book that timestamped its own insertions would be reading one. Two orders that
arrive in the same microsecond need an order anyway; a claimed sequence number
has one, and a replay from an empty book reproduces it exactly.

`the_published_order_is_total_with_no_ties_left` states this as a property
rather than an instance: no two neighbours are ever indistinguishable, so no tie
is ever resolved by something the sequence does not carry.

## Why a sorted `VecDeque` of levels per side

Because the observable property is the order: a side is its levels best
first, a level its orders oldest first, and a test reads that order directly.

Only the front is ever consumed, so the levels sit in a `VecDeque`: an emptied
best level leaves in O(1). A taker sweeping 40 000 one-order levels through
`consume_best` takes 0.9 ms; with a `Vec`, each emptied level shifted every
level behind it, and the same sweep took 452 ms. `insert` finds its position by
`partition_point`.

## Refusals rather than repairs

`cancel` on an absent order returns `None`. It is an ordinary race — the order
may have filled a moment earlier — and a crate that panicked would turn a
routine event into an outage.

`consume_best` past what rests returns `false` and moves nothing, rather than
clamping. A clamp leaves the book and its caller disagreeing about how much
changed hands: books that balance, quantities that do not.

`insert` with nothing left, or more left than the order's own quantity,
returns `false`: a book trading past an order's size moves more than its owner
committed.

`insert` on an id that already rests — on either side — returns `false` and
leaves the book untouched, rather than seating the duplicate. `cancel` only
ever removes the first match for an id, so an unrejected duplicate would
silently outlive its own cancellation and keep trading under a name its owner
believes is gone. The check walks the book only for an id at or below the
highest ever seated; callers minting ids in order never pay for it. Inserting
40 000 orders directly took 443 ms with the walk on every insert, and takes
3.4 ms now.

## Not built: an id index

`Exchange::cancel` finds an order by walking every resting order, `cancel`
walks the instrument's levels for anything behind the front, and
`account_rests` walks the instrument's book. An index from id to instrument,
side and price, plus per-account counts, would make each logarithmic. But
`exchange_core::step_place` clones the whole book on every placement for its
dry run, and a set of ids alone made that clone five times dearer and
placement 48% slower. The index waits for a dry run that does not clone.

## Not built: reducing an order in place

Venues let an order shrink without losing its place
(`exchange_core/docs/algorithm/003_cancel_and_amend_semantics.md`). The book
side is a write to the node's `remaining`; the rest is not here — the
facade has to release the freed reservation in the same step and report it,
and it applies no amend or replace yet.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_id`, `exchange_level`, `exchange_side`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `Book` (per instrument, a sorted `VecDeque` of levels per side) and `Resting` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | Hash order taken for price order; a worse level crossed first |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/priority_test.rs`](tests/priority_test.rs) | Test Matrix T02–T04 — priority from both sides, and cancel |

## Related

- [`exchange_level/`](../exchange_level/readme.md) — one price's FIFO queue; `Resting` is its `LevelNode`
- [`exchange_order/`](../exchange_order/readme.md) — the orders this book holds
- [`exchange_match/`](../exchange_match/readme.md) — consumes the book from the front
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p10_best`, `demo_p11_walk`
