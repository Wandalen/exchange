# Dependency Edge: exchange_depth

### Scope

- **Purpose**: Record which crates `exchange_depth` compiles against.
- **Responsibility**: `exchange_depth` → `exchange_book`.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/017_exchange_depth.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_depth` depends on `exchange_book` alone, reading it without mutating it.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1147` | Dependency edge 11 in Prompt 9's `dependency_edge` list |
