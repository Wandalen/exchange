# Pitfall: Ring overflow as a silent drop

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Letting a full ring silently discard a command instead of surfacing it as a visible rejection.
- **In Scope**: `inbound_ring`'s overflow configuration and `inbound_overflow_reject`'s return value.

### Statement

A dropped order when the ring is full is indistinguishable, to the submitter, from an order that is quietly resting — if overflow is not turned into an explicit rejection, a player or system believes its order is live when it was never accepted at all, and any escrow already taken against it becomes unaccounted for.

### How this crate avoids it

[`inbound_ring`] always builds with `OverflowPolicy::Fail`, never
`RingConfig::new`'s default, `DropNewest`, under which `try_push` returning
`Ok` does not mean the record was kept. It is this crate's only
`with_overflow` call. [`inbound_overflow_reject`] forwards to
`Producer::try_push`, whose `Err` that policy makes reachable.

[`tests/exchange_inbound_test.rs`](../../tests/exchange_inbound_test.rs)'s
`a_full_ring_rejects_the_next_publish_instead_of_dropping_it` fills a
capacity-2 ring and gets the third command back as `Err`.
`smoke_exchange_phases`' `demo_p29_over` shows the same end to end.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/024_ring_overflow_as_silent_drop.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:923` | Pitfall in the source's Prompt 7 "Ring" list |
| [`../../src/lib.rs`](../../src/lib.rs) | `inbound_ring`'s forced `OverflowPolicy::Fail`; module doc's "Overflow is a reject" section |
| [`../../tests/exchange_inbound_test.rs`](../../tests/exchange_inbound_test.rs) | `a_full_ring_rejects_the_next_publish_instead_of_dropping_it` |
