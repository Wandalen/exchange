# ADR-002: Escrow is reached through an `EscrowPort` trait, defined in this crate

**Date**: 2026-10-06
**Status**: Accepted
**Deciders**: User (chose escrow-only, refactor in place, and an `exchange_core`-only edit scope), implementing session

## Context

`Exchange` called the concrete `exchange_escrow::Escrow` directly, so the only
way to make an escrow call fail in a test was to engineer a real balance edge
case — `a_crossing_whose_second_trade_cannot_settle_commits_nothing` in
`tests/submission_test.rs` spends roughly a hundred lines getting one
`settle` to refuse. The facade's own decisions — which escrow call comes
when, and that nothing commits once any of them refuses — could not be
tested apart from `Escrow`'s arithmetic.

The family design already drew a seam at exactly this point: an `EscrowPort`
trait (`hold_try`/`hold_release`/`hold_commit`) for workstream 010 to
implement, with a stub escrow in the wall smoke — see
[`../../../../docs/decision/003_010_implements_escrow_and_consumes_fills.md`](../../../../docs/decision/003_010_implements_escrow_and_consumes_fills.md).
The build collapsed it into the concrete struct.

## Decision

1. **`EscrowPort` lives in `exchange_core`**, with `reserve`/`release`/`settle`
   — the names the code already uses, not the design's `hold_*`. `Escrow`
   implements it by forwarding to its inherent methods.
2. **`Clone` is a supertrait.** `step_place` dry-runs every placement against
   a clone of escrow before replaying it for real, so the port has to be
   cloneable. The trait's doc states the three rules that dry run relies on
   and cannot check: deterministic (clones included), a refused call changes
   nothing, and a clone is fully independent.
3. **The error stays `EscrowError`**, not an associated type, so
   `reason_for`'s translation into a `RejectReason` stays one exhaustive match.
4. **`Exchange< E = Escrow >`.** `new()` and `open_account()` stay on
   `Exchange< Escrow >` only. `new()` because a default type parameter does not
   drive inference — a generic `new` fails every bare `Exchange::new()` with
   E0283 "type annotations needed" (checked on a minimal reproduction, not
   assumed). `open_account()` because funding an account is the ledger's
   concern, not the port's. `with_escrow( E )` is the generic constructor.
5. **Only escrow is behind a trait.** `Book` and `exchange_match::cross` stay
   concrete.

## Alternatives Considered

- **Also put the matcher, or every collaborator, behind a trait.** More seams
  than any current test needs, and a mocked `Book` forces a mocked `cross`
  with it. Rejected by the user's choice of escrow only.
- **Define the trait in `exchange_escrow`, or its own crate**, where the
  family design places it. Rejected under this change's edit scope
  (`exchange_core` only).
- **A transactional port (`begin`/`commit`/`rollback`) instead of `Clone`.**
  Would let a ledger that cannot be cheaply copied implement the port, but
  changes the dry-run shape of `step_place` itself. Not needed while every
  implementation is in memory.

## Consequences

- The facade's sequencing is graded against a scripted escrow in
  `tests/escrow_port_test.rs` — reject-reason mapping, all-or-nothing commit on
  a failing `settle` or `release`, IOC release ordering, one event per applied
  escrow call, and a cancel whose release refuses.
- Every existing caller compiles unchanged: `Exchange::new()` and `Exchange`
  in type position both mean `Exchange< Escrow >`.
- The replay's `expect`s now trust *any* implementation's determinism. That is
  documented on the trait, not enforced by it.
- A future workstream 010 crate implementing the port would have to depend on
  this facade to name the trait. If that happens, move the trait to its own
  crate.
- The family docs' "Design status" lines still say no `EscrowPort` trait
  exists — `docs/escrow_port/*`, `docs/decision/003`,
  `docs/neighbor_contract/003`, `docs/boundary/002_out.md`. Left as they are,
  outside this crate.

## Related

- [`001_submit_replaced_by_ring_fed_exchange_step.md`](001_submit_replaced_by_ring_fed_exchange_step.md) —
  the `step_place` pipeline whose escrow calls this ADR routes through the port.
- `exchange_core::EscrowPort`'s own doc comment — the authoritative statement
  of the port's three rules; this ADR records why, not what.
