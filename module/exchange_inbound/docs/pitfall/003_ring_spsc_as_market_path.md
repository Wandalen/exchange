# Pitfall: ring_spsc as the market path (many producers)

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Never letting more than one producer write to a single ring, on a path that genuinely has many concurrent submitters.
- **In Scope**: `inbound_ring`'s backend selection and the one-ring-per-producer discipline it depends on.

### Statement

The exchange's inbound edge has many producers (every account submitting orders concurrently) feeding one drain — picking `ring_spsc` for that path violates its single-producer contract the moment a second submitter appears, with a failure mode that may not surface until concurrent load actually happens.

### How this crate avoids it

Every ring [`inbound_ring`] builds is a `ring_spsc` ring: it never calls
`with_producers`, `RingConfig`'s default is one producer, and
`ring_core::Ring::new` picks the SPSC backend for one. The pitfall's harm is
a second writer on such a ring, and none can exist: `ring_handle::Producer`
has no `Clone` or `try_clone`, and `Ends::split` yields one pair. Many
producers are many rings
([`../decisions/001_two_producers_is_two_rings.md`](../decisions/001_two_producers_is_two_rings.md)),
which also records why depending on `ring_core`/`ring_mpsc` for a cloneable
producer was rejected.

`grep -n with_producers module/exchange_inbound/src/lib.rs` finds nothing.
`smoke_exchange_phases`' `demo_p28_drain` races two threads, each on its own
ring.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/025_ring_spsc_as_market_path.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:924` | Pitfall in the source's Prompt 7 "Ring" list |
| `../../../../../ring/ring_config/src/lib.rs` | `producers: 1` default, `with_producers`, `is_multi_producer` |
| `../../../../../ring/ring_core/src/lib.rs` | `Ring::new`'s `Spsc`/`Mpsc` backend selection on `is_multi_producer()` |
| `../../../../../ring/ring_handle/src/lib.rs` | `Producer`'s real declaration — no `Clone`, `Ends::split` yields exactly one pair |
| [`../../src/lib.rs`](../../src/lib.rs) | Module doc's "Two producers without two threads on one ring" |
