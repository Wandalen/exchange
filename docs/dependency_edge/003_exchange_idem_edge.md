# Dependency Edge: exchange_idem

### Scope

- **Purpose**: Record which crates `exchange_idem` compiles against.
- **Responsibility**: `exchange_idem` → `exchange_id`.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/011_exchange_idem.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_idem` depends on `exchange_id` alone, to key its seen-order-id set.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1139` | Dependency edge 3 in Prompt 9's `dependency_edge` list |
