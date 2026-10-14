# Dependency Edge: exchange_inbound

### Scope

- **Purpose**: Record which crates `exchange_inbound` compiles against.
- **Responsibility**: `exchange_inbound` → `exchange_order`, `exchange_rest`, `exchange_match`, `exchange_event`, plus `ring_core`/`ring_handle`, `ring_tls`, `ring_flush`, `ring_batch`, `ring_overflow`, `ring_poll`.

**Design status**: Built — `exchange_inbound` is real and carries a real
`ring_*` edge, but on three crates (`ring_factory`, `ring_handle`,
`ring_types`), not the six originally named here. This line was stale: an
earlier pass recorded "zero `ring_*` dependency exists anywhere in the
family" before `exchange_inbound` itself existed; re-verified 2026-10-07 via
the same `[dependencies]`-scoped Cargo.toml inspection used throughout this
corpus and found to no longer hold. See `../ring_edge/001_exchange_inbound_ring_edge.md`
for the full resolved edge and `../crate/022_exchange_inbound.md` for the
crate-level summary.

### Statement

`exchange_inbound` is the one crate meant to carry the `ring_*` edge at all — everything downstream of it (book, match) must stay `ring_*`-free.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1152` | Dependency edge 16 in Prompt 9's `dependency_edge` list |
