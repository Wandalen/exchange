# Dependency Edge: exchange_spec

### Scope

- **Purpose**: Record which crates `exchange_spec` compiles against.
- **Responsibility**: `exchange_spec` → `exchange_id`, workstream 006's types.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/005_exchange_spec.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_spec` depends on `exchange_id` for instrument identity and on workstream 006's types for tick/lot values.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1137` | Dependency edge 1 in Prompt 9's `dependency_edge` list |
