# exchange_fill — manual testing plan

`Trade`'s fields are plain data. The one piece of logic here is
`Trade::executed_price`, whose two parameters are both `Price` — returning the
wrong one compiles. And `EventKind` is consumed in crates this suite cannot
reach, where a wildcard arm would swallow a new variant silently.

## M1 — `executed_price` returning the taker's price is caught

```bash
# In module/exchange_fill/src/lib.rs, in executed_price,
# replace  maker
# with     _taker
cargo test -p exchange_fill --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `a_trade_executes_at_the_makers_price` fails; every other test
still passes.

**Observed 2026-10-03:**

```
test a_trade_executes_at_the_makers_price ... FAILED
test p16_fill_names_maker_and_taker ... ok
test reject_reason_is_still_exhaustively_matchable ... ok
test a_trade_holds_every_field_distinctly ... ok
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Reverted; a second run confirmed 4/4 tests pass again.

## M2 — no `EventKind` consumer has a wildcard arm

```bash
grep -rn 'EventKind::' module/*/src | grep -v '^module/exchange_fill/'
```

**Expected:** every use names its variant; no `_ =>` arm or bare binding
stands in for the rest.

**Observed 2026-10-10:** every use is in `exchange_core/src/lib.rs` — five
constructions, `let EventKind::Trade( trade ) = event.kind else { continue }`
in `postings`, and `matches!( event.kind, EventKind::OrderAccepted { side : Side::Buy, .. } )`
in `parties`. Both readers name one variant; the `..` elides fields, not
variants.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | M1 | Returning the taker's price is caught immediately, with no collateral failures. |
| 2026-10-10 | M2 | No wildcard over `EventKind` anywhere in the family. Moved here from `exchange_types`, which ran it on 2026-09-13 when it still declared `EventKind`. |
