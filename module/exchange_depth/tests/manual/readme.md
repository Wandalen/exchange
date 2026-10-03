# exchange_depth — manual testing plan

The automated suite pins three load-bearing behaviors: same-price
aggregation, top-N truncation, and the `BadN` guard. This plan checks
whether each pin is actually load-bearing rather than redundant.

## M1 — same-price orders actually aggregate

```bash
# In substrate/exchange/module/exchange_depth/src/lib.rs, in levels_top's
# aggregation arm, replace  top.qty = qty_saturating_add( top.qty, r.remaining );
# with  let _ = qty_saturating_add( top.qty, r.remaining );
cargo test -p exchange_depth --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `same_price_orders_aggregate_into_one_level` fails — the
second order's quantity never joins the level's running total.

**Observed 2026-10-02:** exactly that —

```
failures:
    same_price_orders_aggregate_into_one_level

test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; see Run Record.

## M2 — truncation at `n` distinct levels is enforced

```bash
# In levels_top, replace  if levels.len() == n  with  if false
cargo test -p exchange_depth --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `truncates_to_n_distinct_levels` and
`truncation_drops_every_order_behind_the_cut_price` both fail — every
distinct price now joins the result, however many there are.

**Observed 2026-10-02:** exactly that —

```
failures:
    truncates_to_n_distinct_levels
    truncation_drops_every_order_behind_the_cut_price

test result: FAILED. 7 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; see Run Record.

## M3 — `n == 0` is actually refused

```bash
# In depth_top, replace  if n == 0  with  if false
cargo test -p exchange_depth --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** only `zero_n_is_refused` fails — every other test asks for
`n >= 1` and is unaffected.

**Observed 2026-10-02:** exactly that —

```
failures:
    zero_n_is_refused

test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; a final run confirmed 9/9 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-02 | M1, M2, M3 | Disproved-by-mutation that all three pins are redundant — each of three independent mutations (no-op aggregation, disabled truncation, disabled `BadN` guard) is caught immediately, by exactly the tests expected to catch it and no others. |
