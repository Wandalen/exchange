# exchange_match — manual testing plan

Most of `cross_inner`'s loop is already exercised exhaustively by
`tests/crossing_test.rs` and `tests/self_match_test.rs`'s per-branch unit
tests. The one piece of new logic since TIF landed — the FOK gate in
`cross()` that decides whether to commit `cross_inner`'s result against the
real book or discard it — has exactly one load-bearing condition, covered
below.

## M0 — the FOK probe-vs-commit gate is load-bearing

```bash
# In substrate/exchange/module/exchange_match/src/lib.rs, in cross(),
# replace  if would_fill.remaining != Quantity::ZERO
# with     if would_fill.remaining == Quantity::ZERO
cargo test -p exchange_match --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** every FOK test in `tests/tif_test.rs` fails — the gate's sense
is inverted, so a fillable order gets discarded and an unfillable one gets
committed against the real book. IOC tests are untouched, since IOC never
reaches this branch (`tif_requires_full` is false for `Tif::Ioc`).

**Observed 2026-10-03:**

```
test fok_that_cannot_fill_entirely_leaves_the_book_untouched ... FAILED
test fok_short_by_a_second_level_rejects_the_whole_order_and_restores_the_first ... FAILED
test fok_that_exactly_exhausts_two_levels_fills_both ... FAILED
test fok_that_fits_entirely_fills_and_touches_the_real_book ... FAILED
test result: FAILED. 3 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The 3 passing tests in that run are `ioc_fills_partially_and_reports_the_same_remainder_gtc_would`,
`ioc_against_no_liquidity_crosses_nothing`, and `fok_against_no_liquidity_crosses_nothing`
— the last one passes even mutated because an empty book makes `cross_inner`
return `remaining == incoming.quantity` regardless of which comparison gates
the commit, so there is nothing for this particular mutation to disturb on
an empty book either way.

Reverted; a second run confirmed all 7 of `tif_test.rs`'s tests (and the
crate's other 17) pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | M0 | Disproved-by-mutation that the FOK gate's `!=` direction is load-bearing — inverting it breaks every FOK case that has liquidity to reason about, and leaves every IOC case and the liquidity-free FOK case untouched. |
