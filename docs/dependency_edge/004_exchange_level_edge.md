# Dependency Edge: exchange_level

### Scope

- **Purpose**: Record which crates `exchange_level` compiles against.
- **Responsibility**: `exchange_level` → `exchange_order`, `exchange_seq`.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/008_exchange_level.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_level` depends on `exchange_order` for what it stores and `exchange_seq` for FIFO ordering within the price.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1140` | Dependency edge 4 in Prompt 9's `dependency_edge` list |
