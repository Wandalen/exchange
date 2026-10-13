# exchange_match — manual testing plan

The loop's branches are covered by the unit tests. The FOK gate in `cross()`
— commit the probe's result to the real book, or discard it — has one
load-bearing condition, below.

## M0 — the FOK probe-vs-commit gate is load-bearing

```bash
# In module/exchange_match/src/lib.rs, in cross(),
# replace  if would_fill.remaining != Quantity::ZERO
# with     if would_fill.remaining == Quantity::ZERO
cargo test -p exchange_match --all-features --no-fail-fast 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** every FOK test with liquidity to reason about fails — the
gate's sense is inverted, so a fillable order is discarded and an unfillable
one committed against the real book — in `tests/tif_test.rs` and in the T13
cases of `tests/self_match_test.rs`. IOC, GTC and post-only tests are
untouched: they never reach this branch.

**Observed 2026-10-13** (`--no-fail-fast`, so every test binary runs):

```
test t13_a_self_crossing_fok_is_killed_whole_when_the_incoming_side_is_withdrawn ... FAILED
test t13_a_self_crossing_fok_under_cancel_resting_fills_past_its_own_order ... FAILED
test t13_a_self_crossing_fok_that_cannot_fill_leaves_its_own_order_resting ... FAILED
test result: FAILED. 9 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test fok_short_by_a_second_level_rejects_the_whole_order_and_restores_the_first ... FAILED
test fok_that_exactly_exhausts_two_levels_fills_both ... FAILED
test fok_that_cannot_fill_entirely_leaves_the_book_untouched ... FAILED
test fok_that_fits_entirely_fills_and_touches_the_real_book ... FAILED
test result: FAILED. 5 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`fok_against_no_liquidity_crosses_nothing` passes mutated: on an empty book
`cross_inner` returns the whole quantity whichever way the gate points.

Reverted; all 33 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | M0 | Disproved-by-mutation that the FOK gate's `!=` direction is load-bearing — inverting it breaks every FOK case that has liquidity to reason about, and leaves every IOC case and the liquidity-free FOK case untouched. |
| 2026-10-13 | M0 | Same, now including the three T13 FOK self-match cases. |
