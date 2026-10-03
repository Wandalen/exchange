# Pitfall: HashMap iteration as price order

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Treating a hash map's iteration order as if it were sorted price order.
- **In Scope**: `InstrumentBook`'s `bids`/`asks` fields and how `Book` ranks across them.

### Statement

A hash map's iteration order is an implementation detail, not a price order —
code that walks one expecting price-time priority will see a different,
unstable order on every run (and potentially every machine), silently
corrupting the one guarantee a matching engine cannot do without.

### How this crate avoids it

`grep -n "HashMap" module/exchange_book/src/lib.rs` returns zero hits. Each
instrument's own `InstrumentBook` holds `bids`/`asks` as plain `Vec<Level>`
([`src/lib.rs`](../../src/lib.rs)) — highest price first for bids, lowest
first for asks — never a hash-keyed container; price ranking is the Vec's own
sort order, not an incidental side effect of iterating anything. (The type
narrowed from an earlier `Vec<Resting>` to today's `Vec<Level>` when
per-price-level storage moved out to `exchange_level`; the "no hash
iteration" guarantee this pitfall is about held before that move and holds
unchanged after it.) Verified directly:
[`tests/priority_test.rs`](../../tests/priority_test.rs)'s
`t03_price_priority_across_levels_for_bids` and
`t03_price_priority_across_levels_for_asks` insert resting orders out of
price order and assert they come back ranked correctly.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/005_hashmap_iteration_as_price_order.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:898` | Pitfall in the source's Prompt 7 "Book" list |
