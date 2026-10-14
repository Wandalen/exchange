# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here (`Book`/`Resting` from `exchange_book`; `InstrumentId`/`OrderId` from `exchange_id`).

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `rest_place` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `rest_cancel` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `RestReplaceError` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `rest_replace` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

No decisions collection: the narrower error surface than proposed is
explained in [`../item/readme.md`](../item/readme.md).
