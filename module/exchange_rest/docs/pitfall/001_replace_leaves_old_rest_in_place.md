# Pitfall: Replace that leaves the old rest in place

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Implementing cancel-and-replace so the original order is actually gone, not merely shadowed, once the new one rests.
- **In Scope**: `rest_replace`'s cancel-then-insert-with-rollback sequencing.

### Statement

A replace is supposed to remove the old order and rest the new one in its
place; if the old order is left on the book, the account now has two live
orders where it asked for one, doubling its exposure and reservation
without its knowledge.

### How this crate avoids it

[`rest_replace`] never inserts the replacement while the original is still
resting: it calls [`Book::cancel`] on `old_id` *first*, and only then
attempts [`Book::insert`] of `new_resting`. There is no window in which old
and new both rest at once — by the time `insert` runs, `cancel` has
already removed `old_id`, so a successful insert leaves exactly one order
resting, never two.

The ordering also opens a different risk — a refused insert leaving the
book with *zero* orders instead of one — which the rollback branch closes:
on a refused insert, the just-cancelled `old` is reinserted unchanged
before `rest_replace` returns `Err(RestReplaceError::Refused)`, guarded by
a `debug_assert!` noting that an id just freed by `cancel`, with its own
prior-valid `remaining`, cannot itself be refused by `insert`.

**Verified**: [`tests/exchange_rest_test.rs`](../../tests/exchange_rest_test.rs)'s
`rest_replace_swaps_price_and_quantity_atomically` asserts `book.len() == 1`
after a successful replace — "exactly one order rests — not the old plus
the new" — directly ruling out this pitfall's failure mode.
`rest_replace_refused_by_a_colliding_id_restores_the_original` and
`rest_replace_refused_by_zero_quantity_restores_the_original` cover the
rollback branch, asserting `book.len() == 2` and the original's exact
price/remaining survive a refused replace. The rollback branch's
load-bearing-ness is further confirmed by mutation:
[`tests/manual/readme.md`](../../tests/manual/readme.md)'s M0 removed the
rollback `insert` call and observed exactly those two rollback tests fail
(7 passed, 2 failed), with the `Missing`-path test unaffected; reverting
restored all 9 tests plus the doctest to passing.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/009_replace_leaves_old_rest_in_place.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:902` | Pitfall in the source's Prompt 7 "Book" list |
| `../../src/lib.rs` | `rest_replace`'s real cancel-then-insert-with-rollback implementation |
