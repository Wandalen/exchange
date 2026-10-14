# Manual plan — exchange_core

Every check below is a command with an expected observation, run against a
guard this crate's automated suite exercises only indirectly. Checks that
edit a source file say so, and each ends by reverting.

## M1

`Exchange::submit`'s dispose step asserts `self.book.insert(...)` on every
resting order, with the message "order id came from this exchange's own
next_order counter, which never repeats." Grep confirms this is the only
site in the crate that inserts into the book, and the assert has no
`unwrap_or`/`if let Ok` fallback — a failed insert panics rather than
silently dropping the order.

```bash
grep -n "order id came from this exchange" substrate/exchange/exchange_core/src/lib.rs
```

**Expected:** one match, on the `assert!` inside the dispose step.

**Observed 2026-09-13:** `src/lib.rs:357:    "order id came from this exchange's own next_order counter, which never repeats",` — confirmed the sole occurrence via `grep -c` returning 1.

## M2

`tests/contract_test.rs`'s family-wide walk (`the_walk_reaches_every_crate_in_the_family`)
only proves something was checked if `sources()` actually finds files. Its
`pending` seed list is one directory per crate; mutating it to start empty
makes the walk find nothing, which should fail this test's own
`lines.len() > 500` assertion and the sibling `!files.is_empty()` assertion
— not silently pass the two prohibition tests (ECS types, floats) vacuously.

```bash
cd substrate/exchange/exchange_core
sed -i 's/let mut pending = vec!\[ module_dir().*$/let mut pending : Vec< std::path::PathBuf > = vec![];/' tests/contract_test.rs
cargo nextest run -p exchange_core --all-features --no-fail-fast 2>&1 | grep -E "FAIL|no source under|only .* lines walked"
git checkout -- tests/contract_test.rs
```

**Expected:** both `the_walk_reaches_every_crate_in_the_family` and the two
prohibition tests either fail or the walk-completeness assertion fails first,
demonstrating the vacuous-pass guard is load-bearing.

**Observed 2026-09-13:** with `pending` emptied, `t12_no_ecs_type_appears_anywhere_in_the_family` and `no_floating_point_appears_anywhere_in_the_family` PASS vacuously (zero lines to scan), while `the_walk_reaches_every_crate_in_the_family` FAILs with `exchange_types has no source under src/` — exactly the guard test catching the vacuous condition the other two tests couldn't see themselves. Reverted; `cargo nextest run -p exchange_core --all-features` back to 29/29 green (`-9869_exchange_core_m2_revert.log`).

## M3

`claim_order()` increments `self.next_order` on every call specifically so
order ids never repeat within one `Exchange`. The natural guess is that a
repeat would trip the book-insert assert from M1 — but `Escrow::reserve()`
runs *before* the book is ever touched, keyed by `order.id` in a
`HashMap<OrderId, Obligation>`, and rejects a second reservation under an
id already held with `EscrowError::AlreadyReserved`. `reason_for()`'s own
doc comment states why that variant is bundled into the generic
`RejectReason::InsufficientFunds` rather than surfaced on its own: *"the
order id is freshly claimed so nothing could already hold a reservation
under it"* — i.e. the mapping assumes `AlreadyReserved` is unreachable
through `submit`. Breaking id-uniqueness falsifies that assumption before
the mutation ever reaches the assert M1 checks.

```bash
cd substrate/exchange/exchange_core
sed -i '/let id = OrderId( self.next_order );/{n;/self.next_order += 1;/d}' src/lib.rs
cargo nextest run -p exchange_core --all-features --no-fail-fast 2>&1 | grep -c "^     FAIL"
git diff --stat src/lib.rs
git checkout -- src/lib.rs
```

**Expected:** every test submitting a second order to the same `Exchange`
fails at its own `.submit(...).unwrap()` call, not at the book-insert
assert — because the second order's escrow reservation collides with the
first's before the book is reached.

**Observed 2026-09-13:** with the increment removed, 13 of 29 tests FAIL, every one panicking at its own second-or-later `.submit(...).unwrap()` with `Err(Rejected(InsufficientFunds))` (log: `-9870_exchange_core_m3_mutant.log`) — none reach the M1 assert at all, confirming the reservation-layer collision is the real, earlier-firing mechanism, not the book-insert path the guard's own comment implies. Reverted; `cargo nextest run -p exchange_core --all-features` back to 29/29 green (`-9871_exchange_core_m3_revert.log`).

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-09-13 | M1, M2, M3 | 3/3 confirmed — guards load-bearing as documented |
