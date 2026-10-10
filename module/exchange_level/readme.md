# exchange_level

One price, FIFO rest — the queue a book keeps at a single price point.
Depends on `exchange_id`, `exchange_order`, `exchange_seq` and `exact_arith`.

```rust
use exact_arith::{ Price, Quantity };
use exchange_id::{ AccountId, InstrumentId, OrderId };
use exchange_level::{ level_new, level_push, level_pop_front, LevelNode };
use exchange_order::Order;
use exchange_seq::Sequence;
use exchange_side::Side;
use exchange_tif::Tif;

let mut level = level_new( Price::parse( "2.50" ).unwrap() );
let order = Order
{
  id : OrderId( 1 ), instrument : InstrumentId( 1 ), account : AccountId( 1 ),
  side : Side::Buy, price : Price::parse( "2.50" ).unwrap(),
  quantity : Quantity::from_int( 4 ).unwrap(), tif : Tif::Gtc, client : None,
};
level_push( &mut level, LevelNode { order, remaining : order.quantity, arrival : Sequence( 1 ) } );

assert_eq!( level_pop_front( &mut level ).unwrap().order.id, OrderId( 1 ) );
```

## Where it sits

`exchange_book` keeps one `Level` per price per side, best price first; the
level keeps arrival order inside its price. `exchange_book::Resting` is an
alias of `LevelNode`. A partial fill reduces the front node's `remaining` in
place through the public `nodes` field.

## Not the proposal's intrusive list

The source design specifies `Level { price, head, len }` /
`LevelNode { order, next }`, a singly linked list. For push-back and
pop-front, all a sweep needs, no safe node chain — `LinkedList`, `Box` or `Rc`
nodes, an arena linked by index — beats a `VecDeque`: O(1) at the front, so
sweeping a level is linear in its length. O(1) removal mid-queue would take a
doubly linked list plus an id-to-node index; without both, `level_remove`
stays a linear scan. `LevelError { Full,
Missing }` is not built: a resting-order ceiling is
[`exchange_cap`](../exchange_cap/readme.md)'s, checked before a node reaches a
level, and a missing id is `None`, as in `exchange_book::Book::cancel` — see
[`docs/decisions/001_no_level_error.md`](docs/decisions/001_no_level_error.md).

## Not built: pro-rata allocation

Some venues split a fill across a level in proportion to each order's size
rather than by arrival. Proportional shares need rounding to the lot, and the
remainder has to go somewhere — the dust `exact_arith` refuses to create
silently. Allocation is also `exchange_match`'s decision, not the level's.

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
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan — FIFO silently becoming LIFO |

## Related

- [`exchange_book/`](../exchange_book/readme.md) — keeps one `Level` per price per side
- [`exchange_order/`](../exchange_order/readme.md) — `Order`, the type every `LevelNode` wraps
- [`exchange_cap/`](../exchange_cap/readme.md) — the resting-order ceiling this crate does not enforce
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p09_fifo`
