# Dependency Edge: exchange_escrow

### Scope

- **Purpose**: Record which crates `exchange_escrow` compiles against.
- **Responsibility**: `exchange_escrow` → `exchange_id`, `exchange_order`.

**Design status**: The real `exchange_escrow` crate exists and compiles against `exchange_types` (which folds in both `exchange_id` and `exchange_order`) — see `../crate/012_exchange_escrow.md`'s Design status for the fuller scope-expansion account.

### Statement

`exchange_escrow` depends on `exchange_id` for whose account is held and `exchange_order` for what the hold is against.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1142` | Dependency edge 6 in Prompt 9's `dependency_edge` list |
