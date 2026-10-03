# Responsibility: Prices and Quantities From 006

### Scope

- **Purpose**: State that 002 never defines its own numeric types for money or quantity.
- **Responsibility**: Every price and quantity 002 handles is a workstream 006 (exact arithmetic) type.
- **In Scope**: Using `exact_arith`'s `Money`/`Quantity`/`Price` types throughout.
- **Out of Scope**: Defining any new numeric representation, including `f64`.

**Design status**: Held — `exchange_core`'s `Cargo.toml` depends directly on `exact_arith`, and no `f64` usage was found anywhere in the family's public types.

### Statement

002 treats workstream 006 as its sole source of numeric truth for money and quantity, taking no shortcuts with floating point even "just for a demo." The real build honors this fully: every crate's price/quantity-bearing type traces back to `exact_arith`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1003` | Sixth bullet of Prompt 9's `responsibility` list |
