# exchange_cap — manual testing plan

The automated suite pins that a rest at the cap is refused. This plan checks
whether that pin is load-bearing rather than redundant.

## M1 — the rest-cap refusal is load-bearing

```bash
# In module/exchange_cap/src/lib.rs, in cap_check_rest,
# replace the body with  Ok( () )  (and both parameters with  _caps / _current_rests )
cargo test -p exchange_cap --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `a_rest_at_the_cap_is_refused`, `a_zero_cap_refuses_everything`
and `p12_third_rest_is_refused` fail — a rest at the cap is now accepted
instead of refused.

**Observed 2026-10-02:** exactly that —

```
test p12_third_rest_is_refused ... FAILED
test a_rest_at_the_cap_is_refused ... FAILED
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; a second run confirmed 4/4 tests pass again.

**Observed 2026-10-06**, after `a_level_below_the_cap_is_accepted` and
`a_zero_cap_refuses_everything` joined the suite —

```
test a_zero_cap_refuses_everything ... FAILED
test p12_third_rest_is_refused ... FAILED
test a_rest_at_the_cap_is_refused ... FAILED
test result: FAILED. 3 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; 6/6 pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-02 | M1 | Disproved-by-mutation that the rest-cap refusal was redundant — an always-accepts mutation is caught immediately by two independent tests. |
| 2026-10-06 | M1 | Re-run with the extended suite — now caught by three tests. |
