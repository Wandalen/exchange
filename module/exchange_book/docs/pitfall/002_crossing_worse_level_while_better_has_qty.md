# Pitfall: Crossing a worse level while a better one still has qty

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Exposing, or letting a caller consume, any level other than the current best while the best still has quantity resting.
- **In Scope**: `Book::best` and `Book::consume_best`'s own indexing.

### Statement

A taker must exhaust the best price before touching the next one. Skipping to
a worse level while the better one still has quantity hands someone a worse
execution than the book offered.

### How this crate avoids it

By the API's shape. `Book::best` and `Book::consume_best` read and reduce only
the front of the price-sorted `VecDeque<Level>`, and `consume_best` drops that
level only once it is empty. No method takes a level index, so a caller cannot
name the second-best level before the best is gone; `cancel` alone removes
elsewhere, by id.
[`tests/priority_test.rs`](../../tests/priority_test.rs)'s
`a_partly_consumed_order_keeps_its_place` and
`emptied_levels_hand_the_front_to_the_next_best` pin it here;
`exchange_match`'s
[`tests/crossing_test.rs`](../../../exchange_match/tests/crossing_test.rs)
`each_trade_executes_at_its_own_makers_price` and
`the_loop_consumes_in_the_books_published_order` pin it across a sweep.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/008_crossing_worse_level_while_better_has_qty.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:901` | Pitfall in the source's Prompt 7 "Book" list |
