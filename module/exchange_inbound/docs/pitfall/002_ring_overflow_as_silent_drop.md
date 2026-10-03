# Pitfall: Ring overflow as a silent drop

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Letting a full ring silently discard a command instead of surfacing it as a visible rejection.
- **In Scope**: `inbound_ring`'s overflow configuration and `inbound_overflow_reject`'s return value.

### Statement

A dropped order when the ring is full is indistinguishable, to the submitter, from an order that is quietly resting — if overflow is not turned into an explicit rejection, a player or system believes its order is live when it was never accepted at all, and any escrow already taken against it becomes unaccounted for.

### How this crate avoids it

[`inbound_ring`] always builds with `ring_types::OverflowPolicy::Fail` —
never `RingConfig::new`'s own default, `OverflowPolicy::DropNewest`, under
which (per `ring_handle::Producer::try_push`'s own doc comment) "an `Ok` is
not by itself evidence the record was kept." The only `with_overflow` call
site anywhere in this crate is inside [`inbound_ring`] itself, and it always
passes `OverflowPolicy::Fail` — there is no code path that builds a ring any
other way. [`inbound_overflow_reject`] is a thin forward to
`Producer::try_push`, whose `Err` case is reachable *because* of that forced
policy — see [`../../src/lib.rs`](../../src/lib.rs)'s own module doc,
"Overflow is a reject, not a drop — by explicit configuration".

**Verified**: [`tests/exchange_inbound_test.rs`](../../tests/exchange_inbound_test.rs)'s
`a_full_ring_rejects_the_next_publish_instead_of_dropping_it` fills a
capacity-2 ring, then asserts the third `inbound_overflow_reject` call
returns `Err(third)` — the rejected command itself, handed back, not a
silent `Ok`. `smoke_exchange_phases`'s `demo_p29_over.rs` demonstrates the
identical property end to end as P29's own phase-smoke.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/024_ring_overflow_as_silent_drop.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:923` | Pitfall in the source's Prompt 7 "Ring" list |
| [`../../src/lib.rs`](../../src/lib.rs) | `inbound_ring`'s forced `OverflowPolicy::Fail`; module doc's "Overflow is a reject" section |
| [`../../tests/exchange_inbound_test.rs`](../../tests/exchange_inbound_test.rs) | `a_full_ring_rejects_the_next_publish_instead_of_dropping_it` |
