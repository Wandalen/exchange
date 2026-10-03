# Dependency Edge: exchange_inbound

### Scope

- **Purpose**: Record which crates `exchange_inbound` compiles against.
- **Responsibility**: `exchange_inbound` → `exchange_order`, `exchange_rest`, `exchange_match`, `exchange_event`, plus `ring_core`/`ring_handle`, `ring_tls`, `ring_flush`, `ring_batch`, `ring_overflow`, `ring_poll`.

**Design status**: Not built — zero `ring_*` dependency exists anywhere in the family (verified via grep across every `module/*/Cargo.toml`); see `../crate/022_exchange_inbound.md`.

### Statement

`exchange_inbound` is the one crate meant to carry the `ring_*` edge at all — everything downstream of it (book, match) must stay `ring_*`-free.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1152` | Dependency edge 16 in Prompt 9's `dependency_edge` list |
