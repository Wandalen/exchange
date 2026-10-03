# Ring Edge: `exchange_inbound`'s Permitted Ring Dependencies

### Scope

- **Purpose**: Name exactly which `ring_*` crates 002 may depend on, and which it must not, closing off an otherwise open-ended dependency surface.
- **Responsibility**: Six permitted crates, two explicit exclusions, and the rule that only `exchange_inbound` may hold this edge at all.
- **In Scope**: `ring_core` or `ring_handle`, `ring_tls`, `ring_flush`, `ring_batch` (for `drain_order`), `ring_overflow`, `ring_poll`.
- **Out of Scope**: `ring_spsc` (wrong topology — 002 has many producers, not one) and `ring_bench` (a benchmarking tool, not a runtime dependency).

**Design status**: Built (`module/exchange_inbound`), but on three of the six named crates, not all six. The real dependency set is `ring_factory` (construction — `Factory.build::<T>(cfg)` returns the `Split<T>` this family's own module doc names as the only way `build` can be written, since `Split::ends`/`Ends::split` borrow and so cannot also be the thing `build` returns), `ring_handle` (the `Producer`/`Consumer` types `build` eventually yields), and `ring_types` (for `OverflowPolicy` — needed to request `Fail` explicitly, since `RingConfig::new`'s own default is `DropNewest`, which would make the P29 phase-smoke's required rejection silently unreachable). Confirmed by direct read of `ring_factory`/`ring_handle`/`ring_config`/`ring_types`' real source, not re-derived from this doc's own six-crate list.

Three of the original six are not depended on, each for a reason discovered while building, not asserted: `ring_tls` (thread-local staging) has no crate-level equivalent here — `inbound_flush` takes a plain `impl IntoIterator<Item = InboundCmd>` at its call boundary instead, so staging is the caller's concern, not a dependency of this crate. `ring_batch`'s `drain_order` guarantee is not depended on directly because `ring_handle::Consumer::drain()` already supplies the bounded, ordered drain this crate needs, one tier higher — `ring_batch` is `ring_handle`'s own dependency, not something a caller of `ring_handle` needs to also name. `ring_poll` is not needed because nothing here blocks or parks — every operation used (`try_push`, `try_push_batch`, `try_recv`, `drain`) is already non-blocking by construction; `ring_poll`'s whole purpose is guarding a tick path against an operation that *could* park, and there is no such operation here to guard. `ring_core` (the "or" alternative to `ring_handle`) is also not depended on — `ring_factory::Factory::build` already returns a `ring_handle::Split`, so there is no separate `ring_core::Ring` construction step this crate performs itself. `ring_overflow`'s turning-a-full-ring-into-a-`Reject` behavior is real and used, but reached through `ring_config::RingConfig::with_overflow(OverflowPolicy::Fail)` plus `ring_handle::Producer::try_push`'s `Err` return — a configuration value and a method result, not a direct `ring_overflow` dependency edge (that crate is `ring_core`'s own internal implementation detail of the policy, not part of the five-crate export Contract `ring_handle`/`ring_factory` sit on).

### Statement

002's entire relationship to workstream 008 is this one bounded edge, held by `exchange_inbound` alone: either `ring_core` or `ring_handle` as the composed ring type, plus `ring_tls` (thread-local staging), `ring_flush` (moving staged entries into the ring), `ring_batch` (specifically for its `drain_order`, the stable total order guarantee), `ring_overflow` (turning a full ring into a `Reject`), and `ring_poll` (non-blocking progress). `ring_spsc` is excluded by name because 002 has many producers, not one; `ring_bench` is excluded because it is a benchmarking harness, not something a runtime crate depends on.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1194-1202` | Prompt 9's `ring_edge` instance, six permitted plus two excluded |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:80` | Earlier statement of the same edge: "exchange_inbound: ring_core or ring_handle, plus ring_tls, ring_flush, ring_batch, ring_overflow, ring_poll" |
