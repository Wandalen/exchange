# exchange_halt — manual testing plan

The automated suite pins that `halt_set`/`halt_clear` each refuse a no-op
transition rather than silently accepting it. This plan checks whether those
two guards are load-bearing rather than redundant.

## M1 — `halt_set` actually refuses an already-halted instrument

```bash
# In module/exchange_halt/src/lib.rs, in halt_set,
# replace  if spec.halted  with  if false
cargo test -p exchange_halt --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `halt_set_refuses_an_already_halted_spec` fails — a second
halt is now accepted instead of refused.

**Observed 2026-10-03:** exactly that —

```
failures:
    halt_set_refuses_an_already_halted_spec

test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; see Run Record.

## M2 — `halt_clear` actually refuses an unhalted instrument

```bash
# In halt_clear, replace  if !spec.halted  with  if false
cargo test -p exchange_halt --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `halt_clear_refuses_an_unhalted_spec` fails — a resume with
nothing to resume is now accepted instead of refused.

**Observed 2026-10-03:** exactly that —

```
failures:
    halt_clear_refuses_an_unhalted_spec

test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; a final run confirmed 6/6 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | M1, M2 | Disproved-by-mutation that both `Already` guards are redundant — disabling either one is caught immediately, by exactly the test expected to catch it and no others. |
