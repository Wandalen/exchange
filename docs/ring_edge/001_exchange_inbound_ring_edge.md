# Ring Edge: `exchange_inbound`'s Permitted Ring Dependencies

### Scope

- **Purpose**: Name exactly which `ring_*` crates 002 may depend on, and which it must not, closing off an otherwise open-ended dependency surface.
- **Responsibility**: Six permitted crates, two explicit exclusions, and the rule that only `exchange_inbound` may hold this edge at all.
- **In Scope**: `ring_core` or `ring_handle`, `ring_tls`, `ring_flush`, `ring_batch` (for `drain_order`), `ring_overflow`, `ring_poll`.
- **Out of Scope**: `ring_spsc` (wrong topology — 002 has many producers, not one) and `ring_bench` (a benchmarking tool, not a runtime dependency).

**Design status**: Not implemented — zero `ring_*` dependency exists anywhere in `substrate/exchange` (verified via grep across all `module/*/Cargo.toml`).

### Statement

002's entire relationship to workstream 008 is this one bounded edge, held by `exchange_inbound` alone: either `ring_core` or `ring_handle` as the composed ring type, plus `ring_tls` (thread-local staging), `ring_flush` (moving staged entries into the ring), `ring_batch` (specifically for its `drain_order`, the stable total order guarantee), `ring_overflow` (turning a full ring into a `Reject`), and `ring_poll` (non-blocking progress). `ring_spsc` is excluded by name because 002 has many producers, not one; `ring_bench` is excluded because it is a benchmarking harness, not something a runtime crate depends on.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1194-1202` | Prompt 9's `ring_edge` instance, six permitted plus two excluded |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:80` | Earlier statement of the same edge: "exchange_inbound: ring_core or ring_handle, plus ring_tls, ring_flush, ring_batch, ring_overflow, ring_poll" |
