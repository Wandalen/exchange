# exchange_idem — manual testing plan

`idem_insert`'s correctness rests entirely on reading `BTreeSet::insert`'s
return value the right way round (`true` means newly inserted, `false` means
already present) — a convention different collection APIs disagree on, and the
one place a reviewer's intuition could silently flip without the compiler
noticing.

## M1 — the insert-success branch is load-bearing

```bash
# In substrate/exchange/module/exchange_idem/src/lib.rs, in idem_insert,
# replace  if set.seen.insert( id )
# with     if !set.seen.insert( id )
cargo test -p exchange_idem --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** most of the suite fails, not just one isolated test — the
condition is the one branch every other behaviour (seen, remove, re-insert)
is built on top of.

**Observed 2026-10-02:**

```
test result: FAILED. 2 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Failing: `a_forgotten_id_can_be_inserted_again`, `distinct_ids_are_independent`,
`p13_a_repeat_insert_is_refused_as_a_duplicate`,
`removing_a_seen_id_forgets_it_and_reports_true`,
`the_first_insert_of_an_id_succeeds`. Reverted; a second run confirmed 7/7
tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-02 | M1 | Disproved-by-mutation that the insert-success branch was redundant — flipping it breaks 5 of 7 tests, with no collateral change to the other 2 (`a_new_set_has_seen_nothing`, `removing_an_absent_id_reports_false_rather_than_panicking`, which never call `idem_insert`). |
