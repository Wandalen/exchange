# exchange_rest

The three non-matching ways an order moves on the book: rest, cancel,
replace.

```rust
use exact_arith::{ Money, Quantity };
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
  side : Side::Sell, price : Money::parse( price ).unwrap(),
  quantity : Quantity::from_int( quantity ).unwrap(), tif : Tif::Gtc,
};

let mut book = Book::new();
let original = order( 1, "2.50", 4 );
book.insert( Resting { order : original, remaining : original.quantity, arrival : Sequence( 1 ) } );

let replacement = order( 1, "2.60", 6 );
let resting = Resting { order : replacement, remaining : replacement.quantity, arrival : Sequence( 2 ) };
rest_replace( &mut book, InstrumentId( 1 ), OrderId( 1 ), resting ).unwrap();
```

## Thin, not the proposal's full orchestration

`rest_place`/`rest_cancel` are direct call-throughs to [`exchange_book::Book::insert`]/
[`exchange_book::Book::cancel`] — `exchange_core`'s `submit`/`cancel` keep
owning idempotency/cap/escrow/halt checks inline, unchanged, per the family's
Stage 9 decision. See [`src/lib.rs`](src/lib.rs) for the full reasoning.

## `rest_replace` is new, not extracted

No atomic cancel-and-reinsert exists anywhere else in the family — a caller
wanting to replace an order today calls `cancel` then `submit` as two
separate, non-atomic steps. `rest_replace` closes that, with a rollback to
the original on a refused insert rather than leaving the book short an
order. See [`src/lib.rs`](src/lib.rs)'s doc comment for the exact contract.

## Not yet wired into `exchange_core`

Built and tested standalone; consuming it from `Exchange::cancel`/a future
`Exchange::replace` is facade work this crate does not do itself — same
skeleton-first pattern already used for `exchange_cap` and `exchange_conserve`.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_book`, `exchange_id` |
| [`src/lib.rs`](src/lib.rs) | `rest_place`, `rest_cancel`, `rest_replace`, `RestReplaceError` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| [`tests/exchange_rest_test.rs`](tests/exchange_rest_test.rs) | Test Matrix — place/cancel forwarding, replace success and both refusal paths with rollback |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_book/`](../exchange_book/readme.md) — the book this crate wraps, never reimplements
- [`exchange_match/`](../exchange_match/readme.md) — crossing, the one book-entry path this crate deliberately does not cover
- [`exchange_core/`](../exchange_core/readme.md) — the facade that will eventually consume this crate's `rest_cancel`/`rest_replace`
