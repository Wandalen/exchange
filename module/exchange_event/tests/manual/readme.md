# exchange_event — manual testing plan

The automated suite pins that `event_drain` actually takes ownership —
leaving the source empty — rather than merely copying it. This plan checks
whether that pin is load-bearing rather than redundant.

## M1 — `event_drain` actually empties the source

```bash
# In substrate/exchange/module/exchange_event/src/lib.rs, in event_drain,
# replace  core::mem::take( events )  with  events.clone()
cargo test -p exchange_event --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `drain_takes_every_event_in_order_and_empties_the_source` and
`a_second_drain_after_the_first_returns_nothing` both fail — the source is
left intact, so a second drain returns the same events again.

**Observed 2026-10-03:** exactly that —

```
failures:
    a_second_drain_after_the_first_returns_nothing
    drain_takes_every_event_in_order_and_empties_the_source

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; a final run confirmed 5/5 tests pass again (the suite has since
grown by two: `push_appends_and_len_reflects_it`,
`clear_empties_without_returning`).

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | M1 | Disproved-by-mutation that the ownership-taking pin is redundant — a clone-instead-of-take mutation is caught immediately by both tests that depend on the source actually emptying. |
