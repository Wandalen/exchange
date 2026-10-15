# exchange_stp — manual testing plan

The automated suite pins that the three policies have distinct names. This
plan checks whether that pin is load-bearing rather than redundant.

## M1 — the distinct-naming guard is load-bearing

```bash
# In module/exchange_stp/src/lib.rs, in stp_name,
# replace the CancelResting arm's "cancel_resting" with "cancel_incoming"
cargo test -p exchange_stp --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `every_policy_has_a_distinct_name` fails, reporting two
`"cancel_incoming"` entries where three distinct names were expected.

**Observed 2026-10-02:** exactly that —

```
thread 'every_policy_has_a_distinct_name' panicked at tests/exchange_stp_test.rs:25:3
assertion `left == right` failed
  left: ["cancel_incoming", "cancel_incoming", "cancel_both"]
 right: ["cancel_resting", "cancel_incoming", "cancel_both"]
test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; a second run confirmed 3/3 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-02 | M1 | Disproved-by-mutation that the distinct-naming test was redundant — a name collision between two policies is caught immediately. |
