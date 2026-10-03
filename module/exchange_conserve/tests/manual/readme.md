# exchange_conserve — manual testing plan

`fill_legs_sum`'s own arithmetic is a direct `try_fold` over `Money::checked_add`,
already exhaustively covered by exact unit values.

## M0 — which side pushes which leg first is a documented non-finding

Revised for the second design (both legs per trade, pushed together) — the
original M0 (swapping which `Side` negates) no longer applies to this code at
all; this is its replacement, same underlying lesson. Swapping which arm
pushes `(debit, notional)` vs `(notional, debit)` — i.e. `Side::Buy` and
`Side::Sell` trading push orders — was tried first and found to be
**unobservable**, not caught, for an even more direct reason than the first
design's: `legs` is summed by [`fill_legs_sum`], and addition does not care
what order its terms were pushed in. Both arms push the exact same two
values (`debit` and `notional`) regardless of which label they're under; only
their *order* in the `Vec` differs, and sum is order-independent. No test,
however constructed, could ever distinguish them. Confirmed by actually
running it (every test returns the same Ok/Err either way):

```
test a_balanced_leg_set_sums_to_zero ... ok
test a_trade_with_an_inexact_notional_is_refused ... ok
test an_empty_batch_conserves_trivially ... ok
test an_unbalanced_leg_set_sums_to_its_true_imbalance ... ok
test p17_conserve_assert_distinguishes_ok_from_inexact ... ok
test a_mixed_batch_of_different_magnitudes_still_conserves ... ok
test a_single_trade_conserves ... ok
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The `match` on `taker_side` is kept anyway, purely to label which amount is
the buyer's and which the seller's — see `src/lib.rs`'s "What this still
catches" and the module doc's closing paragraph on `postings()` — not because
swapping it changes any observable behavior today.

## M1 — the terminal balance check is load-bearing

```bash
# In substrate/exchange/module/exchange_conserve/src/lib.rs, in conserve_assert,
# replace  if net == Money::ZERO
# with     if net != Money::ZERO
cargo test -p exchange_conserve --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** every `conserve_assert`-level test that exercises an ordinary
(non-inexact) batch fails (the pass/fail condition is inverted);
`fill_legs_sum`-only tests, and the one test whose batch already errs before
reaching this line (`a_trade_with_an_inexact_notional_is_refused`), are
unaffected.

**Observed 2026-10-03:**

```
test a_balanced_leg_set_sums_to_zero ... ok
test a_single_trade_conserves ... FAILED
test an_empty_batch_conserves_trivially ... FAILED
test a_trade_with_an_inexact_notional_is_refused ... ok
test a_mixed_batch_of_different_magnitudes_still_conserves ... FAILED
test an_unbalanced_leg_set_sums_to_its_true_imbalance ... ok
test p17_conserve_assert_distinguishes_ok_from_inexact ... FAILED
test result: FAILED. 3 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`a_trade_with_an_inexact_notional_is_refused` stays green because its batch
already returns `Err` from the `notional()` call, before the mutated line is
ever reached — exactly the isolation the expectation above names.

Reverted; a second run confirmed 7/7 tests (plus both doctests) pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | M0 (original design) | Ran-and-disproved against the first, single-leg-per-trade design: the buy/sell sign split was not load-bearing — provably symmetric under an Ok/Err-only error shape. Superseded by the redesign below. |
| 2026-10-03 | Redesign | `conserve_assert` changed from one signed leg per trade to both legs (debit + credit) per trade, after discovering the first design could never be called correctly from `exchange_match::cross` — every trade in one real batch shares the same `taker_side`, so same-signed legs can never sum to zero. See `src/lib.rs`'s "Revision" section. `ConserveError::Unbalanced` is unreachable through `conserve_assert` under the new design; P17's demo and this suite's tests moved from a magnitude-mismatch example to an inexact-notional one. |
| 2026-10-03 | M0 (redesign) | Ran-and-disproved against the new design: which arm pushes which leg first is not load-bearing — sum is order-independent. |
| 2026-10-03 | M1 (redesign) | Disproved-by-mutation that the terminal zero check is redundant — inverting it breaks every test whose batch reaches that line, and none of the ones that don't. |
