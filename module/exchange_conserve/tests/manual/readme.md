# exchange_conserve — manual testing plan

Summing the legs is `exact_arith::money_sum_assert_zero`'s job, tested in
`exact`; this crate only builds the legs.

## M0 — which arm pushes which leg first is a documented non-finding

Swap the push order of the two arms in `conserve_assert` — `Side::Buy`
pushing `( notional, debit )` and `Side::Sell` `( debit, notional )`. Both
arms push the same two values, and the sum does not depend on their order,
so no test can tell the two apart.

```bash
# In module/exchange_conserve/src/lib.rs, in conserve_assert, swap the
# two legs.push( .. ) calls inside each match arm
cargo test -p exchange_conserve --all-features --test exchange_conserve_test 2>&1 | grep 'test result'
```

**Observed 2026-10-11:** `test result: ok. 6 passed` — unobservable, as
expected. Reverted.

The `match` on `taker_side` stays: it labels which leg is the buyer's, the
shape a fee will need.

## M1 — retired

M1 inverted the terminal zero check. That check now lives in
`exact_arith::money_sum_assert_zero`, so the mutated line no longer exists
here. While it did (2026-10-03), the inversion failed every test whose batch
reached it and none of the others.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | Redesign | One signed leg per trade replaced by both legs: every trade from one `cross` call shares its taker, so single legs never net to zero. See `src/lib.rs`'s "Two legs per trade". |
| 2026-10-03 | M0 | Unobservable — the sum is order-independent. |
| 2026-10-03 | M1 | Inverting the zero check failed 4 of 7 tests, exactly those whose batch reached it. |
| 2026-10-10 | M1 retired | The zero check moved to `exact_arith::money_sum_assert_zero`. |
| 2026-10-10 | M0 | Still unobservable: 6/6 pass with the push order swapped. |
| 2026-10-11 | M0 | Re-run after the docs trim: 6/6 pass. |
