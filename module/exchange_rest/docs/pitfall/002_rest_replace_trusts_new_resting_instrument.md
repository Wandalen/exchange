# Pitfall: `rest_replace` trusts `new_resting`'s own instrument

### Scope

- **Purpose**: Name a specific mistake this crate does not yet guard against.
- **Responsibility**: Documenting that `rest_replace`'s `instrument` parameter is never cross-checked against `new_resting.order.instrument`, honestly, rather than implying a guard that isn't there.
- **In Scope**: `rest_replace`'s own `instrument`/`new_resting` parameter pair.

### Statement

A replace is supposed to move one order, atomically, from its old slot to a
new one on the *same* instrument's book. If a caller passes a `new_resting`
whose own `order.instrument` disagrees with the `instrument` argument,
nothing stops it: the old order is cancelled from `instrument`'s book, and
the new one is inserted onto whatever book `new_resting.order.instrument`
actually names — silently moving an order onto a different instrument's
book entirely, with no error.

### Why this crate does not yet guard against it

[`rest_replace`] uses its `instrument` parameter only for the
[`Book::cancel`] lookup (`book.cancel( instrument, old_id )`). The subsequent
[`Book::insert`] of `new_resting` derives its target instrument internally
from `new_resting.order.instrument` — there is no assertion, `debug_assert`,
or runtime check that the two agree, unlike the `Missing`/`Refused` paths
`rest_replace` already guards carefully (see
[`001_replace_leaves_old_rest_in_place.md`](001_replace_leaves_old_rest_in_place.md)).

This is consistent with — and arguably required by — this crate's own
documented "thin wrapper, no validation" philosophy (see this crate's module
doc, "Thin, not the proposal's full orchestration"): validation is meant to
be a facade-level concern, not this crate's own. But no facade validates it
either, yet: `exchange_core`'s own `exchange_step` never dispatches
`InboundCmd::Replace` to `rest_replace` at all right now (deferred for an
unrelated reason — see
[`../../../exchange_core/docs/decisions/001_submit_replaced_by_ring_fed_exchange_step.md`](../../../exchange_core/docs/decisions/001_submit_replaced_by_ring_fed_exchange_step.md)),
which happens to make this gap unreachable through the facade today, but
does not close it: a direct caller of `rest_replace`, or a future facade
that wires `Replace` through without adding its own instrument check, would
still hit it.

**Not verified, deliberately**: no test in
[`tests/exchange_rest_test.rs`](../../tests/exchange_rest_test.rs) exercises
a mismatched-instrument `rest_replace` call, because there is no guard to
assert against — writing one would either lock in the bug's own silent
behavior as if it were intended, or assert a failure with no fix attached.
Named here as an open gap instead, per this family's own "named rather than
silently absent" convention.

### Sources

| File | Relationship |
|------|--------------|
| `../../src/lib.rs:133-151` | `rest_replace`'s real implementation — the `instrument` parameter's only use is the `cancel` call |
| `../../../exchange_core/docs/decisions/001_submit_replaced_by_ring_fed_exchange_step.md` | The facade-level decision that makes this gap unreachable through `exchange_core` today, without fixing it |
