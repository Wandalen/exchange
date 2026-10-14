# exchange_rest

The three non-matching ways an order moves on the book: rest, cancel,
replace. Depends on `exchange_book`, `exchange_id` and `exchange_side`.
Closes hard problem 9 (cancel and replace) and features 8 (`book_rest`,
`book_cancel`) and 25 (`book_replace`).

```rust
use exact_arith::{ Price, Quantity };
use exchange_book::{ Book, Resting };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_order::Order;
use exchange_rest::rest_replace;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

let order = | id, price : &str, quantity | Order
{
  id : OrderId( id ), instrument : InstrumentId( 1 ), account : AccountId( id ),
  side : Side::Sell, price : Price::parse( price ).unwrap(),
  quantity : Quantity::from_int( quantity ).unwrap(), tif : Tif::Gtc, client : None,
};

let mut book = Book::new();
let original = order( 1, "2.50", 4 );
book.insert( Resting { order : original, remaining : original.quantity, arrival : Sequence( 1 ) } );

let replacement = order( 1, "2.60", 6 );
let resting = Resting { order : replacement, remaining : replacement.quantity, arrival : Sequence( 2 ) };
rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), resting ).unwrap();
```

## Two forwards and one operation of its own

`rest_place` and `rest_cancel` forward to `Book::insert` and `Book::cancel`
and answer as they do. Idempotency, caps, halt and escrow stay with the
caller.

`rest_replace` withdraws one order and rests another in one step. As a cancel
followed by a place, the original is gone before the replacement is checked:
a refused replacement leaves neither resting, and putting the original back
seats it behind every order sharing its arrival. `rest_replace` asks
`Book::insertable` first, counting the original as gone, and touches nothing
on a refusal.

A replacement is the same order, amended: another side, account, time in
force or client id is refused (`NotAnAmend`), as the family's amend rule has
it (`exchange_core/docs/algorithm/003_cancel_and_amend_semantics.md`). The
id may change. Its `arrival` follows the rule's priority table: a larger
quantity or another price arrives after the original, a smaller or equal
quantity at the same price no earlier (`JumpsQueue`). And nothing here
matches, so a replacement priced into the opposite side is refused
(`WouldTake`) rather than rested there. Rested, it would leave the book
crossed: the next order to arrive trades against it while the better price
it crossed sits on.

## Who calls it

`exchange_core::step_place` rests every remainder through `rest_place`.
`Exchange::cancel` cancels on the book itself, releasing escrow in the same
step, and `exchange_step` answers a drained `Replace` with
`StepOutcome::ReplaceNotWired`. `exchange_inbound::inbound_apply` calls all
three.

## Not built: an amend

`rest_replace` swaps orders on the book. An amend also moves the reservation
in the same step and reports it
(`exchange_core/docs/algorithm/003_cancel_and_amend_semantics.md`);
`Exchange` has neither the escrow side nor an event for it, so
`exchange_step` answers `Replace` with `StepOutcome::ReplaceNotWired`.
`rest_replace` refuses an arrival the priority rule forbids; which later
arrival is right is the facade's to say, since it stamps them.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_book`, `exchange_id`, `exchange_side` |
| [`src/lib.rs`](src/lib.rs) | `rest_place`, `rest_cancel`, `rest_replace`, `RestReplaceError` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | A replace leaving the old order resting; a replacement for another instrument |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_rest_test.rs`](tests/exchange_rest_test.rs) | Place and cancel forwarding; replace, each refusal, the original keeping its place |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan — the check before the cancel |

## Related

- [`exchange_book/`](../exchange_book/readme.md) — the book this crate moves orders on
- [`exchange_match/`](../exchange_match/readme.md) — the matching way onto the book
- [`exchange_inbound/`](../exchange_inbound/readme.md) — applies `Cancel` and `Replace` commands through this crate
- [`exchange_core/`](../exchange_core/readme.md) — rests through `rest_place`
