# exchange_book

The resting order book — two sides, each in published order, best at the front.

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

## Closes hard problem 1, and features 6 and 22

`per_instrument` keys every side by `InstrumentId`, so one instrument's
resting orders can never cross another's — this crate closes hard problem 1
(one book per instrument). `Book` itself — one instrument's bid and ask
ladders — is feature 6. Every read path (`side`, `best`, `iter`) walks
`per_instrument` and each side's sorted levels in price order, never a
hash, which is feature 22 (sorted price walk); `exchange_match` and
`exchange_depth` both rely on this without re-sorting anything themselves.

## The contract is the order, not the storage

Matching takes from the front and never re-decides. That makes the *published
order* the whole of what this crate promises: a book holding the right orders in
the wrong sequence fills the wrong people, and nothing downstream can notice —
every trade is well-formed, every balance conserves, and the wrong account got
paid.

So every test in [`tests/priority_test.rs`](tests/priority_test.rs) reads
`side()` rather than any internal, and the ranking rule lives in one private
function that both `insert` and the tests' own re-derivation agree on:

- better price first — higher for bids, lower for asks;
- at one price, earlier arrival first.

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
`partition_point`, but its duplicate-id check still walks the whole book.

## Refusals rather than repairs

`cancel` on an absent order returns `None`. It is an ordinary race — the order
may have filled a moment earlier — and a crate that panicked would turn a
routine event into an outage.

`consume_best` past what rests returns `false` and moves nothing, rather than
clamping. A clamp leaves the book and its caller disagreeing about how much
changed hands: books that balance, quantities that do not.

`insert` on an id that already rests — on either side — returns `false` and
leaves the book untouched, rather than seating the duplicate. `cancel` only
ever removes the first match for an id, so an unrejected duplicate would
silently outlive its own cancellation and keep trading under a name its owner
believes is gone.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_id`, `exchange_level`, `exchange_side`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `Book`, `Resting`, and the one ranking function |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | The 2 "Book" pitfalls that are purely this crate's own storage choice and indexing |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/priority_test.rs`](tests/priority_test.rs) | Test Matrix T02–T04 — priority from both sides, and cancel |

## Related

- [`exchange_order/`](../exchange_order/readme.md) — the orders this book holds
