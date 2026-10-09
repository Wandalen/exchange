# exchange_spec — manual testing plan

`spec_new`'s zero-tick/zero-lot rejection just forwards `exact_snap::Tick::new`/
`Lot::new`'s own already-tested guard — nothing to re-prove there. The one
mistake only this crate's own glue code could make is a same-typed argument
swap the compiler cannot catch: `base`/`quote` are both `AssetId`, so
transposing them in the struct literal still type-checks.

## M1 — the `base`/`quote` argument order is load-bearing

```bash
# In module/exchange_spec/src/lib.rs, in spec_new,
# replace  InstrumentSpec { id, base, quote, tick, lot, halted : false }
# with     InstrumentSpec { id, base : quote, quote : base, tick, lot, halted : false }
cargo test -p exchange_spec --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `base_and_quote_are_not_swapped` fails; every other test still
passes (the mutation is isolated, not a broad break).

**Observed 2026-10-02:**

```
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test base_and_quote_are_not_swapped ... FAILED
test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Reverted; a second run confirmed 8/8 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-02 | M1 | Disproved-by-mutation that `base_and_quote_are_not_swapped` was redundant — a transposed assignment is caught immediately, with no collateral failures. |
