# Pitfall: Replace that leaves the old rest in place

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Implementing cancel-and-replace so the original order is gone, not shadowed, once the new one rests.
- **In Scope**: `rest_replace`'s check-then-cancel-then-insert sequencing.

### Statement

A replace removes the old order and rests the new one. If the old order stays
on the book, the account has two live orders where it asked for one, doubling
its exposure without its knowledge.

### How this crate avoids it

[`rest_replace`](../../src/lib.rs) never inserts while the original rests: it
cancels `old_id` first, then inserts `new_resting`, so a successful replace
leaves exactly one of them.

That order opens the opposite risk — a refused insert leaving neither. So
`Book::insertable`, asked with `old_id` counted as gone, runs before the
cancel, and a replacement `insert` would refuse returns
`RestReplaceError::Refused` with nothing touched. Cancelling and putting the
original back instead loses its place among orders sharing its arrival.

[`tests/exchange_rest_test.rs`](../../tests/exchange_rest_test.rs):
`rest_replace_swaps_price_and_quantity_atomically` checks one order rests
after a replace; `rest_replace_refused_by_a_colliding_id_leaves_the_original`,
`rest_replace_refused_by_zero_quantity_leaves_the_original` and
`rest_replace_refused_keeps_the_originals_place_among_shared_arrivals` check
that a refused replace leaves the original's price, remaining and queue place.
[`tests/manual/readme.md`](../../tests/manual/readme.md)'s M0 removes the
check and sees exactly those three fail.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/009_replace_leaves_old_rest_in_place.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:902` | Pitfall in the source's Prompt 7 "Book" list |
