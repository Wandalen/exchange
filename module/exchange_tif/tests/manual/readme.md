# exchange_tif — manual testing plan

The automated suite pins which dispositions rest and which require a complete
fill. This plan checks whether that pin is load-bearing rather than redundant.

## M1 — the GTC-only-rests guard is load-bearing

```bash
# In module/exchange_tif/src/lib.rs, in tif_rests,
# replace  matches!( tif, Tif::Gtc | Tif::PostOnly )  with  true  (and the parameter with  _tif )
cargo test -p exchange_tif --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `only_gtc_and_post_only_rest` and `p03_tif_disposition` both
fail — IOC now reports as resting, which neither test accepts. (Before
post-only, the first test was `only_gtc_rests`.)

**Observed 2026-10-02:** exactly that —

```
test p03_tif_disposition ... FAILED
test only_gtc_rests ... FAILED
test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; a second run confirmed 3/3 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-02 | M1 | Disproved-by-mutation that the GTC-only-rests guard was redundant — an always-rests mutation is caught immediately by two independent tests. |
