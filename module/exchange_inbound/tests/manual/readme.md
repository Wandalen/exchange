# exchange_inbound — manual testing plan

The unit tests cover the ring round trip and `inbound_apply`'s dispatch. Two
conditions carry the crate's guarantees and are checked by mutation below:
the overflow policy `inbound_ring` sets, and the book check a `Place` meets
before it crosses. The two-thread race is `smoke_exchange_phases`' P28.

## M0 — the overflow policy is load-bearing

```bash
# In module/exchange_inbound/src/lib.rs, in inbound_ring(), replace
#   with_overflow( OverflowPolicy::Fail )
# with
#   with_overflow( OverflowPolicy::DropNewest )
cargo test -p exchange_inbound --all-features --no-fail-fast 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** the three tests that fill a ring fail. Under `DropNewest` a
full ring takes every push and drops it: the third publish comes back `Ok`,
and a flush counts commands the consumer never sees.

**Observed 2026-10-14:**

```
test a_full_ring_rejects_the_next_publish_instead_of_dropping_it ... FAILED
test a_flush_leaves_what_did_not_fit_with_the_caller ... FAILED
test flush_accepts_as_many_as_fit_and_reports_the_count ... FAILED
test result: FAILED. 22 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## M1 — the book check before crossing is load-bearing

```bash
# In module/exchange_inbound/src/lib.rs, in inbound_apply()'s Place arm,
# delete the  if !book.insertable( &resting, None ) { ... }  block.
cargo test -p exchange_inbound --all-features --no-fail-fast 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** the three duplicate-id tests fail. A place under a resting id
crosses first; where its remainder would rest, the `assert!` on
`rest_place` fires.

**Observed 2026-10-14:**

```
test a_duplicate_place_id_is_refused_before_it_trades ... FAILED
test a_place_whose_id_rests_is_refused_before_it_trades ... FAILED
test duplicate_place_id_is_refused_not_silently_dropped ... FAILED
test result: FAILED. 22 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Reverted after each; all 25 tests pass again.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-14 | M0, M1 | Each mutation breaks exactly the tests named above and nothing else. |
