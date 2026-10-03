# Pitfall: f64 prices or quantities

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Using `f64`/`f32` for any price or quantity, even in a throwaway demo.
- **In Scope**: Money

**Design status**: Avoided — `exchange_core` depends on `exact_arith` directly; no `f64`/`f32` appears in any price or quantity type in the real crates.

### Statement

Reaching for a binary float "just for the demo" is the mistake this names — once a float value exists anywhere in the pipeline, rounding error and binary-exploitable price boundaries follow it into real code paths later, because a demo's types rarely get replaced before they get depended on.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:892` | Pitfall in the source's Prompt 7 "Money" list |
