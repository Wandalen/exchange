# Dependency Edge: exchange_stats

### Scope

- **Purpose**: Record which crates `exchange_stats` compiles against.
- **Responsibility**: `exchange_stats` → `exchange_id`.

**Design status**: Most of these crates don't exist separately in the real build — see `../crate/021_exchange_stats.md`'s Design status for which real crate (if any) now contains both ends of this edge.

### Statement

`exchange_stats` depends on `exchange_id` alone, to key its counters.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1151` | Dependency edge 15 in Prompt 9's `dependency_edge` list |
