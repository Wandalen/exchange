# exchange_rest — manual testing plan

`rest_place`/`rest_cancel` are one-line call-throughs to `Book::insert`/
`Book::cancel`, already exhaustively covered by `exchange_book`'s own test
suite — nothing here to mutate that isn't that crate's own coverage.
`rest_replace`'s refusal check is this crate's one genuinely new piece of
logic.

## M0 — the check before the cancel is load-bearing

```bash
# In module/exchange_rest/src/lib.rs, in rest_replace, delete the
# `if !book.insertable( &new_resting, Some( old_id ) ) { ... }` block.
cargo test -p exchange_rest --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** the three tests that refuse a replacement fail — the original
is cancelled and `insert` refuses the replacement — and every other test is
unaffected.

**Observed 2026-10-12:**

```
test rest_replace_refused_by_a_colliding_id_leaves_the_original ... FAILED
test rest_replace_refused_keeps_the_originals_place_among_shared_arrivals ... FAILED
test rest_replace_refused_by_zero_quantity_leaves_the_original ... FAILED
test result: FAILED. 8 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Reverted; all 11 tests plus the doctest pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | M0 | Disproved-by-mutation that the rollback call in `rest_replace`'s refusal branch is load-bearing — removing it breaks exactly the two tests that observe the original order's survival, and nothing else. |
| 2026-10-12 | M0 | The rollback is gone; re-targeted at the `insertable` check that replaced it. Removing it breaks exactly the three refusal tests. |
