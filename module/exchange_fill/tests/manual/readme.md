# exchange_fill — manual testing plan

`Trade`'s fields are plain data with no derived invariant to break. The one
piece of real logic this crate carries is `Trade::executed_price`, and its
two parameters are both `Price` — the same type — so returning the wrong one
is a compiler-invisible mistake.

## M1 — `executed_price` returning the taker's price is load-bearing

```bash
# In substrate/exchange/module/exchange_fill/src/lib.rs, in executed_price,
# replace  maker
# with     _taker
cargo test -p exchange_fill --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `a_trade_executes_at_the_makers_price` fails; every other test
still passes (the mutation is isolated to one function, not a broad break).

**Observed 2026-10-03:**

```
test a_trade_executes_at_the_makers_price ... FAILED
test p16_fill_names_maker_and_taker ... ok
test reject_reason_is_still_exhaustively_matchable ... ok
test a_trade_holds_every_field_distinctly ... ok
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Reverted; a second run confirmed 4/4 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | M1 | Disproved-by-mutation that the maker-price rule was unenforced — returning the taker's price instead is caught immediately, with no collateral failures. |
