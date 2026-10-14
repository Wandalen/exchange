# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here (`InstrumentId`/`OrderId`/`AccountId`, `Side`, `Tif`, `Price`/`Quantity` are all imported, not declared).

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Amount` | type alias | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Order` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Obligation` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Why there is no `order_new`/`order_qty_set`/`order_qty_left`/`order_is_empty`
or `OrderError` despite the source design naming all five:
[`../decisions/readme.md`](../decisions/readme.md).
