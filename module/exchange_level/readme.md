# exchange_level

One price, FIFO rest — the queue a book keeps at a single price point.

```rust
use exact_arith::{ Money, Quantity };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_level::{ level_new, level_push, level_pop_front, LevelNode };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

let mut level = level_new( Money::parse( "2.50" ).unwrap() );
let order = Order
{
  id : OrderId( 1 ), instrument : InstrumentId( 1 ), account : AccountId( 1 ),
  side : Side::Buy, price : Money::parse( "2.50" ).unwrap(),
  quantity : Quantity::from_int( 4 ).unwrap(), tif : Tif::Gtc,
};
level_push( &mut level, LevelNode { order, remaining : order.quantity, arrival : Sequence( 1 ) } );

assert_eq!( level_pop_front( &mut level ).unwrap().order.id, OrderId( 1 ) );
```

## Why this exists

`exchange_book`'s priority rule is "price first, then arrival." Before this
crate, arrival order within one price was implicit — same-price orders just
happened to sit next to each other in a flat sorted `Vec`. `Level` gives that
implicit ordering an explicit home: one price, one FIFO queue, in its own
crate with its own tests.

## Divergence from the source proposal

The proposal specifies `Level { price, head, len }` / `LevelNode { order, next }`
— an intrusive singly-linked list. Rust has no safe way to express a
self-referential node chain without `unsafe`, `Box`, or `Rc<RefCell<_>>`, and
nothing in this family uses any of the three. This crate uses `Vec<LevelNode>`
instead — `level_push` appends at the back, `level_pop_front` removes index
`0` — which costs no more than `exchange_book`'s own pre-retrofit code already
paid doing the same removal one layer up.

`LevelError { Full, Missing }` is not built. `Full` belongs to
[`exchange_cap`](../exchange_cap/readme.md) (a resting-order ceiling, checked
before a node ever reaches a level); `Missing` is handled as `Option`,
matching `exchange_book::Book::cancel`'s own precedent for "not found by id."
Full reasoning: [`docs/decisions/001_no_level_error.md`](docs/decisions/001_no_level_error.md).

See `src/lib.rs`'s module doc for the full reasoning, including why
[`Level::nodes`] stays a public field.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_id`, `exchange_order`, `exchange_seq`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `Level`/`LevelNode` and the seven `level_*` functions |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why there is no `LevelError` |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_level_test.rs`](tests/exchange_level_test.rs) | FIFO order, id removal, qty sum, emptiness |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan — the one same-typed-field mutation risk |

## Related

- [`exchange_book/`](../exchange_book/readme.md) — retrofit to hold `Vec<Level>` per side instead of a flat `Vec<Resting>`
- [`exchange_order/`](../exchange_order/readme.md) — `Order`, the type every `LevelNode` wraps
- [`exchange_cap/`](../exchange_cap/readme.md) — the resting-order ceiling this crate does not enforce
