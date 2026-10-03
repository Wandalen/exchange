# Dependency Edge: exchange_match

### Scope

- **Purpose**: Record which crates `exchange_match` compiles against.
- **Responsibility**: `exchange_match` → `exchange_book`, `exchange_escrow`, `exchange_fill`, `exchange_stp`, `exchange_tif`, `exchange_conserve`.

**Design status**: The real `exchange_match` crate exists and compiles against `exchange_book`, `exchange_escrow` (indirectly via `exchange_core`), and `exchange_types` — see `../crate/016_exchange_match.md`'s Design status; `exchange_tif` has no real counterpart to depend on.

### Statement

`exchange_match` depends on the book it crosses, escrow for holds, fill types for its output, STP for self-trade handling, TIF for order disposition, and conserve for the zero-sum check.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1146` | Dependency edge 10 in Prompt 9's `dependency_edge` list |
