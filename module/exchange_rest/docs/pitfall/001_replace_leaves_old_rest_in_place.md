# Pitfall: Replace that leaves the old rest in place

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Implementing cancel-and-replace so the original order is actually gone, not merely shadowed, once the new one rests.
- **In Scope**: `rest_replace`'s check-then-cancel-then-insert sequencing.

### Statement

A replace is supposed to remove the old order and rest the new one in its
place; if the old order is left on the book, the account now has two live
orders where it asked for one, doubling its exposure and reservation
without its knowledge.

### How this crate avoids it

[`rest_replace`] never inserts the replacement while the original is still
resting: it calls [`Book::cancel`] on `old_id` *first*, and only then
[`Book::insert`] of `new_resting`. There is no window in which old
and new both rest at once — by the time `insert` runs, `cancel` has
already removed `old_id`, so a successful insert leaves exactly one order
resting, never two.

The ordering also opens a different risk — a refused insert leaving the
book with *zero* orders instead of one. [`Book::insertable`], asked with
`old_id` counted as gone, closes it before the cancel: a replacement
`insert` would refuse returns `Err(RestReplaceError::Refused)` with nothing
touched. Cancelling and reinserting the original instead lost its place
among orders sharing its arrival.

**Verified**: [`tests/exchange_rest_test.rs`](../../tests/exchange_rest_test.rs)'s
`rest_replace_swaps_price_and_quantity_atomically` asserts `book.len() == 1`
after a successful replace — "exactly one order rests — not the old plus
the new" — directly ruling out this pitfall's failure mode.
`rest_replace_refused_by_a_colliding_id_leaves_the_original`,
`rest_replace_refused_by_zero_quantity_leaves_the_original` and
`rest_replace_refused_keeps_the_originals_place_among_shared_arrivals`
cover a refused replace: the original's price, remaining and queue place
survive. [`tests/manual/readme.md`](../../tests/manual/readme.md)'s M0
removes the `insertable` check and observes exactly those three fail.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/009_replace_leaves_old_rest_in_place.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:902` | Pitfall in the source's Prompt 7 "Book" list |
| `../../src/lib.rs` | `rest_replace`'s real check-then-cancel-then-insert implementation |
