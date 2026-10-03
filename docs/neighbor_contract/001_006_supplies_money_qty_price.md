# Neighbor Contract: 006 Supplies Money, Qty, Price

### Scope

- **Purpose**: State exactly what workstream 006 (exact arithmetic) provides to 002.
- **Responsibility**: `Money`, `Qty`, `Price` types, the snap (tick/lot rounding) operation, and the conservation check.
- **In Scope**: Type and function supply only.
- **Out of Scope**: 002 never reimplements any of these.

### Statement

Workstream 006 is 002's only source of numeric types and the two operations that keep them honest: snapping a raw value to a legal tick/lot, and asserting that a set of fill legs conserves to zero. 002 calls these; it does not reimplement tick snapping or conservation logic itself (see `pitfall/` category "Money").

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1088` | First bullet of Prompt 9's `neighbor_contract` list |
