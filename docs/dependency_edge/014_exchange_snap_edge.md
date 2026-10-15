# Dependency Edge: exchange_snap

### Scope

- **Purpose**: Record which crates `exchange_snap` compiles against.
- **Responsibility**: `exchange_snap` → `exchange_order`, `exchange_book`.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/020_exchange_snap.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_snap` depends on `exchange_order` for the row shape and `exchange_book` for what it walks to build the snapshot.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1150` | Dependency edge 14 in Prompt 9's `dependency_edge` list |
