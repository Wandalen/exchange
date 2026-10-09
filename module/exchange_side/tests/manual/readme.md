# exchange_side — manual testing plan

The automated suite pins the opposite relation. This plan checks whether that
pin is load-bearing rather than redundant.

## M1 — the opposite relation is load-bearing

```bash
# In module/exchange_side/src/lib.rs, in Side::opposite,
# replace  Self::Buy => Self::Sell  with  Self::Buy => Self::Buy
cargo test -p exchange_side --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `every_side_has_exactly_one_opposite` and `p02_side_opposite`
both fail, reporting `Sell` expected where `Buy` was returned.

**Observed 2026-10-02:** exactly that —

```
thread 'every_side_has_exactly_one_opposite' panicked at tests/exchange_side_test.rs:9:3
assertion `left == right` failed
  left: Buy
 right: Sell
thread 'p02_side_opposite' panicked at tests/exchange_side_test.rs:38:3
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; a second run confirmed 4/4 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-02 | M1 | Disproved-by-mutation that the opposite-relation test was redundant — a `Buy`-to-`Buy` mutation is caught immediately by two independent tests. |
