# Pitfall: Drain order taken from thread completion, not drain_order

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Letting the order in which producer threads finish decide the processing order, instead of a combine order fixed ahead of time.
- **In Scope**: `inbound_ring`'s one-ring-per-producer construction and the fixed external combine order it requires of callers.

### Statement

Whichever producer thread happens to finish first is not a deterministic property — relying on thread-completion order instead of a total, pre-decided drain order means two runs of the identical input can process orders in a different sequence, breaking the determinism the whole design exists to guarantee.

### How this crate avoids it

A ring has one producer: `ring_handle::Producer` has no `Clone` or
`try_clone`, and `Ends::split` yields one pair. So inside a ring there is no
second producer to race. Many producers are many rings, one
[`inbound_ring`] each, and the caller concatenates their
[`inbound_drain`]s in a lane order fixed ahead of time
([`../decisions/001_two_producers_is_two_rings.md`](../decisions/001_two_producers_is_two_rings.md)),
each once its producer has stopped. A drain takes what was published
before the call, so draining a live lane brings the timing back in.

[`tests/exchange_inbound_test.rs`](../../tests/exchange_inbound_test.rs)'s
`a_pushed_command_drains_in_the_same_order_it_was_pushed` pins FIFO through
one ring. The two-thread race itself runs in `smoke_exchange_phases`'
`demo_p28_drain` and `smoke_exchange_book`'s `ring_determinism`: two runs,
one checksum. This crate's tests spawn no threads.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/022_drain_order_from_thread_completion.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:921` | Pitfall in the source's Prompt 7 "Ring" list |
| `../../../../../ring/ring_handle/src/lib.rs` | `Producer`'s real declaration — no `Clone`, `Ends::split` yields exactly one pair |
| `../../../smoke_exchange_phases/src/bin/demo_p28_drain.rs` | The real two-OS-thread race and fixed-order combine this pitfall's full avoidance rests on |
