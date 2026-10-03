# Dependency Edge: exchange_fill

### Scope

- **Purpose**: Record which crates `exchange_fill` compiles against.
- **Responsibility**: `exchange_fill` → `exchange_id`, `exchange_order`.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/013_exchange_fill.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_fill` depends on `exchange_id` and `exchange_order` to name who and what each outcome concerns.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1143` | Dependency edge 7 in Prompt 9's `dependency_edge` list |
