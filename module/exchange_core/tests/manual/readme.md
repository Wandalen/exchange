# Manual plan — exchange_core

Every check below is a command with an expected observation, run against a
guard this crate's automated suite exercises only indirectly. Checks that
edit a source file say so, and each ends by reverting. Commands run from the
crate directory, `module/exchange_core`.

## M1

`Exchange::exchange_step`'s placement rests a remainder through
`exchange_rest::rest_place`, inside an `assert!` with the message "order id
came from this exchange's own next_order counter, which never repeats."
Grep confirms this is the only site in the crate that puts an order on the
book, and the assert has no `unwrap_or`/`if let Ok` fallback — a refused
insert panics rather than silently dropping the order.

```bash
grep -n "order id came from this exchange" src/lib.rs
grep -n "rest_place(\|\.insert(" src/lib.rs
```

**Expected:** one match for the message, on the `assert!` around
`rest_place`; the only other `.insert(` calls are into `specs` and `caps`.

**Observed 2026-10-14:** `src/lib.rs:1020`, the sole occurrence (`grep -c`
returns 1); `rest_place` is called once, at `:1019`; the other inserts are
`self.specs` (`:559`) and `self.caps` (`:573`).

## M2

`tests/contract_test.rs`'s family-wide walk (`the_walk_reaches_every_crate_in_the_family`)
only proves something was checked if `sources()` actually finds files. Its
`pending` seed is one crate's `src/`; mutating it to start empty makes the
walk find nothing, which should fail this test's own `!files.is_empty()`
assertion — not silently pass the two prohibition tests (ECS types, floats)
vacuously.

```bash
perl -i -pe 's/let mut pending = vec!\[ family_dir\(\).*$/let mut pending : Vec< PathBuf > = vec![];/' tests/contract_test.rs
cargo nextest run -p exchange_core --all-features --no-fail-fast -E 'binary(contract_test)' 2>&1 | grep -E "PASS|FAIL|no source under"
git checkout -- tests/contract_test.rs
```

**Expected:** the two prohibition tests pass vacuously, and
`the_walk_reaches_every_crate_in_the_family` fails, demonstrating the
vacuous-pass guard is load-bearing.

**Observed 2026-10-14:** with `pending` emptied,
`t12_no_ecs_type_appears_anywhere_in_the_family` and
`no_floating_point_appears_anywhere_in_the_family` PASS (zero lines to scan),
while `the_walk_reaches_every_crate_in_the_family` FAILs with
`exchange_id has no source under src/` — 6 of 7 pass, 1 fails. Reverted.

## M3

`claim_order()` increments `self.next_order` on every call so order ids never
repeat within one `Exchange`. The natural guess is that a repeat would trip
the M1 assert — but escrow's `reserve()` runs *before* the book is touched,
keyed by `order.id` in a `BTreeMap<OrderId, Reservation>`, and refuses a
second reservation under an id already held with
`EscrowError::AlreadyReserved`. `reason_for()` folds that variant into
`RejectReason::InsufficientFunds`, on the reasoning that "the order id is
freshly claimed so nothing could already hold a reservation under it."
Breaking id uniqueness falsifies that assumption before the mutation reaches
the M1 assert — with `exchange_escrow::Escrow`. An `EscrowPort` that does not
refuse a repeated id leaves the M1 assert as the only guard.

```bash
perl -0 -i -pe 's/(let id = OrderId\( self\.next_order \);\n)\s*self\.next_order \+= 1;\n/$1/' src/lib.rs
git diff --stat src/lib.rs
cargo nextest run -p exchange_core --all-features --no-fail-fast 2>&1 | grep -E "Summary|panicked at" -A1
git checkout -- src/lib.rs
```

**Expected:** a test placing a second order on the same `Exchange` fails at
its own placement, refused with `Rejected(InsufficientFunds)`, not at the M1
assert; a test on an escrow that does not refuse a repeated id reaches the M1
assert.

**Observed 2026-10-14:** 33 of 69 tests FAIL. 28 are refused with
`Rejected(InsufficientFunds)` (26 at an `unwrap()`, 2 at a placement check);
the rest fail downstream of a refused placement (`NotResting`, assertion
mismatches). One reaches the M1 assert:
`escrow_port_test::a_settle_failure_mid_crossing_commits_nothing`, whose
`ScriptedEscrow` does not refuse a repeated id. Reverted.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-09-13 | M1, M2, M3 | 3/3 confirmed — guards load-bearing as documented (against `Exchange::submit`, 29 tests) |
| 2026-10-14 | M1, M2, M3 | 3/3 confirmed against `exchange_step`; M3 also found the M1 assert is the guard under an `EscrowPort` that does not refuse a repeated id |
