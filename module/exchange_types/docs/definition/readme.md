# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here (`Order`/`Obligation` from `exchange_order`, `Side` from `exchange_side`, `Price`/`Quantity`/`Money` from `exact_arith`).

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `TypeError` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `notional` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `obligation` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
