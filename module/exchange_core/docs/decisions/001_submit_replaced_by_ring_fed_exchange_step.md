# ADR-001: `submit` is replaced by a ring-fed `exchange_step`, not kept alongside it

**Date**: 2026-10-03
**Status**: Accepted
**Deciders**: User (explicit choice between two framed options), implementing session

## Context

Stage 9 of the workspace decomposition plan said `exchange_step` goes "in
place of" the old direct `Exchange::submit( account, side, price, quantity )`.
That phrase was ambiguous against the real state of the test suite:
`submit` was exercised by ~45 call sites across `tests/submission_test.rs`,
`smoke_exchange_core`, and `smoke_exchange_phases/src/bin/demo_p07_order.rs`,
none of which Stage 10 is scoped to touch. Two readings were possible —
add `exchange_step` alongside the untouched `submit`, or delete `submit`
outright and migrate every caller. The user was asked directly and chose
the second: delete `submit` entirely, and migrate every caller to the
ring-fed path.

Building `exchange_step` also exposed a design gap `exchange_inbound`'s own
module doc had already anticipated but left unresolved: a ring-fed
`InboundCmd::Place` carries whatever `id`/`arrival` its producer gave it, but
`Exchange` is the only place in the family that knows those values must be
globally unique and monotonic. Something has to decide whether to trust the
producer or reassign.

## Decision

1. **`submit` is deleted, not kept.** `exchange_step( consumer, policy )`
   drains an `exchange_inbound` ring and applies every `InboundCmd` it finds,
   in drain order. There is no second, parallel entry point — every caller
   that wants to place or cancel an order now goes through the ring.

2. **`exchange_step` owns sequencing.** It never trusts a drained
   `InboundCmd::Place`'s own `order.id`/the enclosing `Resting.arrival` —
   it reassigns both itself, via the same `claim_order()`/`emit()` counters
   `submit` used internally. This is what lets many producers push
   provisional, not-yet-authoritative values without coordinating with each
   other: `Exchange` is the single-threaded authority that makes them real on
   drain.

3. **`Cancel` delegates to the existing `Exchange::cancel`, unchanged.**
   `InboundCmd::Cancel`'s own `instrument` field is never consulted —
   `cancel`'s own global, instrument-agnostic search is already correct and
   already tested, so a ring-fed cancel gets exactly the same escrow-release
   behaviour a direct call would, with no second implementation to keep in
   sync.

4. **`Replace` drains but is not yet applied.** Every drained `Replace`
   comes back as `StepOutcome::ReplaceNotWired`. A faithful replace has to
   move the old order's reservation and the new order's obligation
   atomically alongside the book-level swap `exchange_rest::rest_replace`
   already performs, and report it with an event whose `reserved`/`released`
   fields are honest. Neither exists yet: `exchange_fill::EventKind` has no
   "replaced" variant (confirmed by a full read — only
   `OrderAccepted`/`OrderRejected`/`Trade`/`OrderCancelled` exist, and the
   first and last each require a real `Obligation` a book-only replace would
   have none of), and `exchange_rest`'s own module doc already names escrow
   orchestration as a facade-level concern it deliberately left open. Building
   that now would be new orchestration beyond what this stage asked for.

5. **`step_place` reimplements the reserve/cross/settle/rest pipeline
   directly**, via `exchange_match::cross` + `exchange_rest::rest_place`,
   rather than calling `exchange_inbound::inbound_apply`. `inbound_apply` is
   deliberately escrow-free by its own module doc's design — reusing it here
   would mean building a second, parallel escrow-orchestration layer around
   it rather than one. `exchange_core` reimplements the pipeline using the
   same underlying primitives `inbound_apply` itself calls, which is the
   anticipated shape, not a duplication.

## Alternatives Considered

- **Keep `submit` alongside `exchange_step`.** Smaller and fully reversible —
  no test rewrite — but leaves two entry points with the same responsibility,
  one of which (`submit`) could silently drift out of sync with the ring-fed
  path's sequencing/TIF/multi-instrument/self-match-policy behaviour. Rejected
  by the user's own explicit choice.
- **Build full escrow-aware `Replace` now.** Would resolve the deferred item
  immediately, but requires inventing both a new `EventKind` variant and new
  atomic old-release/new-reserve escrow orchestration that nothing has asked
  for yet — a YAGNI violation against a stage whose own scope never named
  Replace as a requirement.
- **Call `inbound_apply` from `step_place` for `Place`.** Would avoid
  reimplementing the reserve/cross/settle/rest sequence, but `inbound_apply`
  has no escrow parameter at all — making it escrow-aware would change its
  own contract for every other caller of that crate, not just this one.

## Consequences

- `tests/submission_test.rs`, `smoke_exchange_core/src/lib.rs`, and
  `smoke_exchange_phases/src/bin/demo_p07_order.rs` all gained a small
  ring-setup helper and now push through `inbound_flush`/`exchange_step`
  instead of calling `submit` directly. Every existing assertion kept its
  original meaning — this was a mechanical migration, not a behavioural one.
- Time-in-Force, multi-instrument routing, and self-match policy all become
  real, caller-chosen values as a direct consequence of routing through the
  ring — not a separate effort. `exchange_core` gained `exchange_tif` as a
  dependency again (and re-exports `Tif`) specifically so a caller can name
  `Tif::Gtc` without reaching past the facade.
- `exchange_rest::rest_replace`'s instrument-mismatch bug (flagged separately
  by the `refactor-exchange-crates-alignment` peer session) is unreachable
  through the facade at this stage, by construction: `Replace` is never
  dispatched to `rest_replace` from `exchange_step`. This does not fix the
  bug in `exchange_rest` itself — a direct caller of `rest_replace` can still
  hit it — but it does mean no path through `exchange_core` can trigger it
  today.

## Related

- `exchange_inbound`'s own module doc, "What this crate does not do" — names
  sequencing as an accept-time decision this crate only carries, anticipating
  this ADR's point 2.
- `exchange_rest`'s own module doc — names escrow orchestration as a
  facade-level concern, the basis for this ADR's point 4.
- [`../../../exchange_rest/docs/pitfall/002_rest_replace_trusts_new_resting_instrument.md`](../../../exchange_rest/docs/pitfall/002_rest_replace_trusts_new_resting_instrument.md) —
  the `rest_replace` instrument-mismatch gap this ADR's point 4 makes
  unreachable through the facade, without fixing it in `exchange_rest` itself.
- `exchange_core::Exchange::exchange_step`'s own doc comment — the
  authoritative, current description of all four points; this ADR records
  why, not what.
