# Dependency Edge: exchange_book

### Scope

- **Purpose**: Record which crates `exchange_book` compiles against.
- **Responsibility**: `exchange_book` → `exchange_level`, `exchange_spec`, `exchange_id`.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/009_exchange_book.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_book` depends on `exchange_level` for its ladders, `exchange_spec` for the instrument's own grid, and `exchange_id` for identity.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1141` | Dependency edge 5 in Prompt 9's `dependency_edge` list |
