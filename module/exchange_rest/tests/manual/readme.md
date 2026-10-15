# exchange_rest — manual testing plan

`rest_place`/`rest_cancel` forward to `Book::insert`/`Book::cancel`, covered
by `exchange_book`'s own suite. `rest_replace`'s check before the cancel is
this crate's own logic.

## M0 — the check before the cancel is load-bearing

```bash
# In module/exchange_rest/src/lib.rs, in rest_replace, delete the
# `if !book.insertable( &new_resting, Some( old_id ) ) { ... }` block.
cargo test -p exchange_rest --all-features --no-fail-fast 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** the three tests that refuse a replacement through `insert`'s
checks fail, and nothing else. The colliding id is now caught later as
`NotAnAmend` (it names another account); the other two reach the cancel and
then the `assert!` on the refused insert.

**Observed 2026-10-14:**

```
test rest_replace_refused_by_a_colliding_id_leaves_the_original ... FAILED
test rest_replace_refused_by_zero_quantity_leaves_the_original ... FAILED
test rest_replace_refused_keeps_the_originals_place_among_shared_arrivals ... FAILED
test result: FAILED. 15 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Reverted; all 18 tests plus the doctest pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | M0 | Disproved-by-mutation that the rollback call in `rest_replace`'s refusal branch is load-bearing — removing it breaks exactly the two tests that observe the original order's survival, and nothing else. |
| 2026-10-12 | M0 | The rollback is gone; re-targeted at the `insertable` check that replaced it. Removing it breaks exactly the three refusal tests. |
| 2026-10-13 | M0 | Same three, after the `NotAnAmend` and `WouldTake` checks landed. |
| 2026-10-14 | M0 | Same three, after the `JumpsQueue` and client-id checks landed. |
