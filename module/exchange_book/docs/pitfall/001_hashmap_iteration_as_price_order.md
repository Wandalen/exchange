# Pitfall: HashMap iteration as price order

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Treating a hash map's iteration order as if it were sorted price order.
- **In Scope**: `InstrumentBook`'s `bids`/`asks` fields and how `Book` ranks across them.

### Statement

A hash map's iteration order is not a price order. Code walking one for
price-time priority sees a different order from run to run, and from machine
to machine.

### How this crate avoids it

`grep -n "HashMap" module/exchange_book/src/lib.rs` returns nothing. Each
instrument's `bids`/`asks` are a sorted `VecDeque<Level>` — highest price
first for bids, lowest first for asks — and the instruments themselves a
sorted `Vec`. Price ranking is the deque's own order.
[`tests/priority_test.rs`](../../tests/priority_test.rs)'s
`t03_price_priority_across_levels_for_bids` and
`t03_price_priority_across_levels_for_asks` insert out of price order and read
the ranking back.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/005_hashmap_iteration_as_price_order.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:898` | Pitfall in the source's Prompt 7 "Book" list |
