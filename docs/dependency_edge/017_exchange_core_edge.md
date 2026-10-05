# Dependency Edge: exchange_core

### Scope

- **Purpose**: Record which crates `exchange_core` compiles against.
- **Responsibility**: `exchange_core` → `exchange_spec`, `exchange_book`, `exchange_rest`, `exchange_match`, `exchange_depth`, `exchange_halt`, `exchange_event`, `exchange_snap`, `exchange_stats`, `exchange_inbound`.

**Design status**: The real `exchange_core` crate depends on `exchange_types`, `exchange_book`, `exchange_fill`, `exchange_match`, `exchange_order`, `exchange_side`, `exchange_escrow`, and `exact_arith` directly (the last three added when `exchange_types`' own re-export aggregator role retired) — narrower than the ten-crate edge proposed here, though several of those ten (`spec`, `rest`, `depth`, `halt`, `event`, `snap`, `stats`, `inbound`) do have a real counterpart by now; see `../crate/023_exchange_core.md` for the fuller, more current accounting.

### Statement

`exchange_core` is the facade — proposed to depend on all ten of the crates below it so the caller never has to.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1153` | Dependency edge 17 in Prompt 9's `dependency_edge` list |
