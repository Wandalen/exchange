# Dependency Edge: exchange_conserve

### Scope

- **Purpose**: Record which crates `exchange_conserve` compiles against.
- **Responsibility**: `exchange_conserve` → `exchange_fill`, workstream 006's types.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/014_exchange_conserve.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_conserve` depends on `exchange_fill` for what it sums and on workstream 006's types for exact arithmetic.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1144` | Dependency edge 8 in Prompt 9's `dependency_edge` list |
