# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `InstrumentId` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `OrderId` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `AccountId` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `ClientOrderId` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `instrument_from_raw`/`instrument_raw` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `order_from_raw`/`order_raw` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `account_from_raw`/`account_raw` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `client_from_raw`/`client_raw` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Why there is no `IdError`: [`../decisions/readme.md`](../decisions/readme.md).
