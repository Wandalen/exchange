# exchange_types — manual testing plan

The automated suite pins `notional`'s exact-or-refused arithmetic. This plan
checks that the refusal is load-bearing: replacing it with silent truncation
is the "simplification" a later reader is most likely to make.

## M1 — the inexact-notional guard is load-bearing

```bash
# In module/exchange_types/src/lib.rs, in notional(),
# replace the condition  if product % scale != 0  with  if false
cargo test -p exchange_types --all-features 2>&1 | grep -E 'FAILED|test result|left|right'
```

**Expected:** `a_notional_needing_rounding_is_refused` fails by returning `Ok`
with a truncated amount — the guard's absence is not caught downstream, it is
laundered into a wrong number.

**Observed 2026-10-10:**

```
test a_notional_needing_rounding_is_refused ... FAILED
  left: Ok(Decimal { minor: Minor(0) })
 right: Err(NotionalInexact)
test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

One minor unit of quantity at one minor unit of price — twelve decimal places
against the currency's six — truncated to zero instead of refused. Reverted;
a second run confirmed 8/8 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-09-13 | M1–M3 | M1 as above, against 11 tests. M2 (executed price) and M3 (`EventKind` wildcard sweep) moved to `exchange_fill` with the types they check. |
| 2026-10-10 | M1 | Re-run against the current 8 tests: the guard is still the only thing between a dust trade and a silent zero. |
