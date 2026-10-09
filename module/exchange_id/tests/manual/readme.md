# exchange_id — manual testing plan

The automated suite pins each id type's raw round-trip. This plan checks
whether that pin is load-bearing rather than redundant.

## M1 — the raw round-trip guard is load-bearing

```bash
# In module/exchange_id/src/lib.rs, in order_raw,
# replace  id.0  with  0  (and the parameter with  _id  to silence the warning)
cargo test -p exchange_id --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `every_id_survives_a_raw_round_trip` and `p01_id_round_trip`
both fail, reporting `7` expected where `0` was returned.

**Observed 2026-10-02:** exactly that —

```
thread 'p01_id_round_trip' panicked at tests/exchange_id_test.rs:45:3
thread 'every_id_survives_a_raw_round_trip' panicked at tests/exchange_id_test.rs:14:3
assertion `left == right` failed
  left: 0
 right: 7
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; a second run confirmed 4/4 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-02 | M1 | Disproved-by-mutation that the round-trip test was redundant — a constant-returning `order_raw` is caught immediately by two independent tests. |
