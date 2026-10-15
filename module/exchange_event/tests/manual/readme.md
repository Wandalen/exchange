# exchange_event — manual testing plan

The automated suite pins that `event_drain` takes the events — leaving the
source empty — rather than copying them. This plan checks that the pin is
load-bearing.

## M1 — `event_drain` actually empties the source

```bash
# In module/exchange_event/src/lib.rs, in event_drain,
# replace  core::mem::take( events )  with  events.clone()
cargo test -p exchange_event --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `drain_takes_every_event_in_order_and_empties_the_source` and
`a_second_drain_after_the_first_returns_nothing` fail — the source is left
intact, so a second drain returns the same events again.

**Observed 2026-10-11:** exactly that —

```
failures:
    a_second_drain_after_the_first_returns_nothing
    drain_takes_every_event_in_order_and_empties_the_source

test result: FAILED. 3 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; 5/5 pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | M1 | Caught by both tests that need the source emptied (1 passed, 2 failed of 3). |
| 2026-10-11 | M1 | Re-run on the current code: same two fail, 3 of 5 pass. |
