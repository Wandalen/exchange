# Dependency Edge: exchange_event

### Scope

- **Purpose**: Record which crates `exchange_event` compiles against.
- **Responsibility**: `exchange_event` → `exchange_fill`.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/019_exchange_event.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_event` depends on `exchange_fill` alone, since every drained event wraps a fill, reject, or cancel-ack.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1149` | Dependency edge 13 in Prompt 9's `dependency_edge` list |
