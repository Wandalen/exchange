# Pitfall: Crossing a worse level while a better one still has qty

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Exposing, or letting a caller consume, any level other than the current best while the best still has quantity resting.
- **In Scope**: `Book::best` and `Book::consume_best`'s own indexing.

### Statement

A taker must exhaust every unit available at the best price before
touching the next-best one; skipping ahead to a worse level while the
better one is not yet empty hands the taker (or the resting side) a worse
execution than the book actually offered.

### How this crate avoids it

This is foreclosed by this crate's own API shape rather than left to a
caller's discipline. `Book::best(instrument, side)` and
`Book::consume_best(instrument, side, taken)` ([`src/lib.rs`](../../src/lib.rs))
only ever read or mutate `levels.front()`/`levels.front_mut()` — the front
of the price-sorted `VecDeque<Level>` — and `consume_best` removes that level
only once its own `remaining` reaches zero
(`level_pop_front`/`levels.pop_front()`), never before. There is no method on
`Book` that takes a level index or otherwise exposes any level but the
current front one; a caller cannot reach the second-best level until
`consume_best` has driven the best one to empty, because nothing in this
crate's surface lets it name the second-best level at all.
`grep -n "front(\|front_mut(\|pop_front(" module/exchange_book/src/lib.rs`
shows every selection and consumption point in the crate going through the
front only; `cancel` alone removes elsewhere, by id. Verified directly:
[`tests/priority_test.rs`](../../tests/priority_test.rs)'s
`a_partly_consumed_order_keeps_its_place` (a level reduced but not drained
stays at the front — the next `best()` call returns the same level, not a
worse one) and `a_fully_consumed_order_leaves_the_book` (a level is removed
only once fully drained) together show the only way to advance past the
current best level is to empty it first. The scenario actually spanning
*two* price levels — a taker large enough to need both, each paid at its
own price in strictly cheapest-first order — is exercised one layer up, by
this crate's own consumer: `exchange_match`'s
[`tests/crossing_test.rs`](../../../exchange_match/tests/crossing_test.rs)
`each_trade_executes_at_its_own_makers_price` and
`the_loop_consumes_in_the_books_published_order`, both of which rely on
nothing more than repeated `best()`/`consume_best()` calls — this crate's
own primitives — to get the ordering right.

### Redistribution note

Previously bundled, without individual review, into this crate's own
pitfall readme's blanket "Out of Scope: the other five 'Book' pitfalls
(006–010)" note, which characterized that whole range as "arrival/FIFO
ordering within one price level — `exchange_level`'s own storage." That
characterization does not hold for this one: pitfall 008 is about ordering
*across* price levels during a cross, not arrival order *within* one level,
and `exchange_level` has no notion of "a worse level" at all (a `Level`
doesn't know about any other `Level`). Re-verified against real source for
this batch and redistributed here on its own; 006, 007, 009, and 010 are
left untouched in that bundle, not reviewed as part of this task.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/008_crossing_worse_level_while_better_has_qty.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:901` | Pitfall in the source's Prompt 7 "Book" list |
