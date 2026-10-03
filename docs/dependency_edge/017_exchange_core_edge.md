# Dependency Edge: exchange_core

### Scope

- **Purpose**: Record which crates `exchange_core` compiles against.
- **Responsibility**: `exchange_core` → `exchange_spec`, `exchange_book`, `exchange_rest`, `exchange_match`, `exchange_depth`, `exchange_halt`, `exchange_event`, `exchange_snap`, `exchange_stats`, `exchange_inbound`.

**Design status**: The real `exchange_core` crate depends on `exchange_types`, `exchange_book`, `exchange_match`, `exchange_escrow`, and `exact_arith` directly — a much narrower edge than proposed, since most of the listed dependencies (`spec`, `rest`, `depth`, `halt`, `snap`, `stats`, `inbound`) have no real counterpart to depend on. See `../crate/023_exchange_core.md`.

### Statement

`exchange_core` is the facade — proposed to depend on all ten of the crates below it so the caller never has to.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1153` | Dependency edge 17 in Prompt 9's `dependency_edge` list |
