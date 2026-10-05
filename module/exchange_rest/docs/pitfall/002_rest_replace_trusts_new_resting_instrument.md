# Pitfall: `rest_replace` trusted `new_resting`'s own instrument (fixed — BUG-001)

### Scope

- **Purpose**: Record a mistake this crate *did* make, found and closed as BUG-001, so the reasoning survives as a lesson rather than vanishing once the diff landed.
- **Responsibility**: Documenting that `rest_replace`'s `instrument` parameter was never cross-checked against `new_resting.order.instrument` until the fix below — historical record, not a live warning.
- **In Scope**: `rest_replace`'s own `instrument`/`new_resting` parameter pair.

### Statement (as it was)

A replace is supposed to move one order, atomically, from its old slot to a
new one on the *same* instrument's book. If a caller passed a `new_resting`
whose own `order.instrument` disagreed with the `instrument` argument,
nothing stopped it: the old order was cancelled from `instrument`'s book,
and the new one was inserted onto whatever book `new_resting.order.instrument`
actually named — silently moving an order onto a different instrument's
book entirely, with no error.

### Why the gap existed, and why the fix lives inside `rest_replace`

[`rest_replace`] used its `instrument` parameter only for the
[`Book::cancel`] lookup (`book.cancel( instrument, old_id )`). The subsequent
[`Book::insert`] of `new_resting` derives its target instrument internally
from `new_resting.order.instrument` — there was no assertion, `debug_assert`,
or runtime check that the two agreed, unlike the `Missing`/`Refused` paths
`rest_replace` already guards carefully (see
[`001_replace_leaves_old_rest_in_place.md`](001_replace_leaves_old_rest_in_place.md)).

This crate's own documented "thin wrapper, no validation" philosophy (see
its module doc, "Thin, not the proposal's full orchestration") argues for
deferring validation to a facade — but that framing is about not
consolidating `exchange_core`'s *external-state* orchestration here
(idempotency sets, cap registries, the escrow ledger). An instrument-match
check needs none of that: both values it compares (`instrument`,
`new_resting.order.instrument`) are already this function's own parameters.
Deferring a check on a function's own two arguments to some future caller
would have left every caller — including `exchange_inbound::inbound_apply`'s
real, already-tested `InboundCmd::Replace` arm, confirmed during BUG-001's
filing to forward both values straight through with no check of its own —
exposed until that caller remembered to add one. Fixed at the leaf instead:
`rest_replace` now asserts the two agree before touching the book at all,
returning `RestReplaceError::InstrumentMismatch` on disagreement.

**Verified**: [`tests/exchange_rest_test.rs`](../../tests/exchange_rest_test.rs)'s
`rest_replace_refuses_a_new_resting_for_a_different_instrument` and
`exchange_inbound`'s own
`replace_refuses_a_new_resting_for_a_different_instrument` both exercise the
mismatch directly. Full investigation, root cause, and fix diff:
[`../../../../../task/exchange_rest/bug/completed/001_rest_replace_trusts_instrument.md`](../../../../../task/exchange_rest/bug/completed/001_rest_replace_trusts_instrument.md)
— relocated to the External Task Layout (`substrate/task/exchange_rest/bug/`,
outside this repo); `completed/` is BUG-001's terminal location, but
re-resolve via `substrate/task/exchange_rest/bug/readme.md`'s own index if
this link ever goes stale regardless.

### Sources

| File | Relationship |
|------|--------------|
| `../../src/lib.rs:62-72,133-151` | `rest_replace`'s real implementation, pre- and post-fix — the `InstrumentMismatch` check now sits first, before `cancel` |
| `../../../../../task/exchange_rest/bug/completed/001_rest_replace_trusts_instrument.md` | BUG-001 — full Hypothesis/Evidence/Root Cause/Fix Location record (External Task Layout, outside this repo) |
| `../../../exchange_core/docs/decisions/001_submit_replaced_by_ring_fed_exchange_step.md` | The facade-level decision that made this gap unreachable through `exchange_core` specifically — never a reason the gap was safe elsewhere |
