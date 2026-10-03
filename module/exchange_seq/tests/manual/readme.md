# exchange_seq — manual testing plan

The automated suite pins that `seq_next` strictly increases. This plan checks
whether that pin is load-bearing rather than redundant.

## M1 — the increment is load-bearing

```bash
# In substrate/exchange/module/exchange_seq/src/lib.rs, in seq_next,
# replace  Sequence( current.0 + 1 )  with  current
cargo test -p exchange_seq --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** all three tests fail — `seq_next` now returns its input
unchanged, which none of them accept.

**Observed 2026-10-02:** exactly that —

```
test seq_next_is_strictly_increasing ... FAILED
test p08_seq_next_monotonic ... FAILED
test zero_is_the_starting_value ... FAILED
test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; a second run confirmed 3/3 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-02 | M1 | Disproved-by-mutation that the monotonicity tests were redundant — a non-incrementing `seq_next` is caught immediately by all three tests. |
