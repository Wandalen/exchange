# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `HaltError` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `halt_set` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `halt_clear` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `halt_is` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Why there is no `exchange_book` dependency despite the source design naming
one: [`../decisions/001_no_exchange_book_dependency.md`](../decisions/001_no_exchange_book_dependency.md).
