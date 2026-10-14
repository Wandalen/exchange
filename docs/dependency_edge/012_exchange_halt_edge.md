# Dependency Edge: exchange_halt

### Scope

- **Purpose**: Record which crates `exchange_halt` compiles against.
- **Responsibility**: `exchange_halt` → `exchange_spec`, `exchange_book`.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/018_exchange_halt.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_halt` depends on `exchange_spec` for the instrument's halt flag and `exchange_book` for what stays resting while halted.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1148` | Dependency edge 12 in Prompt 9's `dependency_edge` list |
