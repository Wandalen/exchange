# Pitfall: Drain order taken from thread completion, not drain_order

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Letting the order producer threads happen to finish in determine processing order, instead of a fixed, pre-decided combine order.
- **In Scope**: `inbound_ring`'s one-ring-per-producer construction and the fixed external combine order it requires of callers.

### Statement

Whichever producer thread happens to finish first is not a deterministic property — relying on thread-completion order instead of a total, pre-decided drain order means two runs of the identical input can process orders in a different sequence, breaking the determinism the whole design exists to guarantee.

### How this crate avoids it

Two guarantees, one structural and one architectural — neither is a
`ring_mpsc`-style shared-ring claim ordering:

**Structural — a ring can never have two producers to race in the first
place.** The `Producer` this crate hands callers (re-exported from
`ring_handle`) has no `Clone`/`try_clone` at all — confirmed directly in
`ring_handle`'s own source (see Sources) — and `Ends::split` borrows `&'a
mut self` to yield exactly one `Producer`/`Consumer` pair. So within one
ring built by [`inbound_ring`], there is never a second producer to race
against the first; "thread completion order" has nothing to act on inside a
single ring.

**Architectural — many producers means many rings, combined in a fixed
order decided ahead of time.**
[`../decisions/001_two_producers_is_two_rings.md`](../decisions/001_two_producers_is_two_rings.md)
records the decision directly: each producer gets its own exclusively-owned
ring via its own [`inbound_ring`] call, genuinely raced from its own OS
thread, with the combine step (concatenating each lane's [`inbound_drain`]
result in a fixed lane order, e.g. "ring A fully, then ring B") living at
the call site rather than inside this crate's own API.

**Verified**: within this crate's own test suite,
[`tests/exchange_inbound_test.rs`](../../tests/exchange_inbound_test.rs)'s
`a_pushed_command_drains_in_the_same_order_it_was_pushed` proves single-ring
FIFO survives the ring — the foundation the fixed-order combine is built
on. The full two-real-thread property (same checksum across two runs of a
genuine two-thread race) is empirically demonstrated one crate up, in
`smoke_exchange_phases`'s `demo_p28_drain.rs` — that binary is the one
place an actual `thread::scope` race against two `inbound_ring`-built rings
exists today; this crate's own test suite does not itself spawn threads.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/022_drain_order_from_thread_completion.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:921` | Pitfall in the source's Prompt 7 "Ring" list |
| `../../../../../ring/ring_handle/src/lib.rs` | `Producer`'s real declaration — no `Clone`, `Ends::split` yields exactly one pair |
| `../../../smoke_exchange_phases/src/bin/demo_p28_drain.rs` | The real two-OS-thread race and fixed-order combine this pitfall's full avoidance rests on |
