# Dependency Edge: exchange_rest

### Scope

- **Purpose**: Record which crates `exchange_rest` compiles against.
- **Responsibility**: `exchange_rest` → `exchange_book`, `exchange_idem`, `exchange_cap`, `exchange_escrow`, `exchange_spec`.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/015_exchange_rest.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_rest` depends on the book it places into, idempotency and capacity checks, escrow for the hold, and spec for grid validation.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1145` | Dependency edge 9 in Prompt 9's `dependency_edge` list |
