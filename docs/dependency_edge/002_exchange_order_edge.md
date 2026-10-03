# Dependency Edge: exchange_order

### Scope

- **Purpose**: Record which crates `exchange_order` compiles against.
- **Responsibility**: `exchange_order` → `exchange_id`, `exchange_side`, `exchange_tif`.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/006_exchange_order.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_order` depends on `exchange_id` for identity, `exchange_side` for direction, and `exchange_tif` for time-in-force.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1138` | Dependency edge 2 in Prompt 9's `dependency_edge` list |
