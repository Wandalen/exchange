# exchange_stats — manual testing plan

The automated suite pins that each counter accumulates independently and that
a snapshot doesn't alias the live counter. This plan checks whether those pins
are load-bearing rather than redundant.

## M1 — `stats_fill_add` actually mutates the counter

```bash
# In module/exchange_stats/src/lib.rs, in stats_fill_add,
# replace the body with  let _ = ( stats, n );
cargo test -p exchange_stats --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `adds_accumulate_independently` and
`snapshot_does_not_alias_the_live_counter` both fail — fills stops
accumulating, so both end up asserting a `fills` count that never arrived.

**Observed 2026-10-02:** exactly that —

```
failures:
    adds_accumulate_independently
    snapshot_does_not_alias_the_live_counter

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; see Run Record.

## M2 — `stats_snapshot` actually copies the live state, not a fixed value

```bash
# In module/exchange_stats/src/lib.rs, in stats_snapshot,
# replace the body with  let _ = stats; BookStats::default()
cargo test -p exchange_stats --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** only `snapshot_does_not_alias_the_live_counter` fails — it's
the one test that actually reads a snapshot's contents; the other two never
call `stats_snapshot` at all.

**Observed 2026-10-02:** exactly that —

```
failures:
    snapshot_does_not_alias_the_live_counter

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; a final run confirmed 3/3 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-02 | M1, M2 | Disproved-by-mutation that both pins are redundant — a no-op `stats_fill_add` and an input-ignoring `stats_snapshot` are each caught immediately, by exactly the tests expected to catch them and no others. |
