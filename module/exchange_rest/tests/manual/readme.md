# exchange_rest — manual testing plan

`rest_place`/`rest_cancel` are one-line call-throughs to `Book::insert`/
`Book::cancel`, already exhaustively covered by `exchange_book`'s own test
suite — nothing here to mutate that isn't that crate's own coverage.
`rest_replace`'s rollback is this crate's one genuinely new piece of logic.

## M0 — the rollback-on-refusal branch is load-bearing

```bash
# In substrate/exchange/module/exchange_rest/src/lib.rs, in rest_replace,
# delete the two lines before `Err( RestReplaceError::Refused )`:
#   let restored = book.insert( old );
#   debug_assert!( restored, "..." );
# leaving the else-branch as just `Err( RestReplaceError::Refused )`.
cargo test -p exchange_rest --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** the two tests that assert the original order comes back after
a refused replacement fail; every other test — including the `Missing` path,
which never reaches this branch — is unaffected.

**Observed 2026-10-03:**

```
test rest_replace_refused_by_a_colliding_id_restores_the_original ... FAILED
test rest_replace_refused_by_zero_quantity_restores_the_original ... FAILED
test result: FAILED. 7 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Reverted; a second run confirmed all 9 tests (plus the doctest) pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | M0 | Disproved-by-mutation that the rollback call in `rest_replace`'s refusal branch is load-bearing — removing it breaks exactly the two tests that observe the original order's survival, and nothing else. |
