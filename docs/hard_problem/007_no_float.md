# Hard Problem: No Float

### Scope

- **Purpose**: Only minor units from 006.
- **Responsibility**: Represent every price and quantity as a 006 minor-unit type, never a binary float.

**Design status**: Held — `exchange_core` depends on `exact_arith` (workstream 006) directly; no `f64`/`f32` appears in any price/quantity type.

### Statement

Binary float is exploitable, and workstream 006 already solved this — every price and quantity here must be a 006 minor-unit type. Without it, the book accumulates rounding drift.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:217-220` | Hard problem 7 in the source's Prompt 1 answer for workstream 002 |
