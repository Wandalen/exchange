# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `BookStats` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `stats_zero` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `stats_rest_add` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `stats_fill_add` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `stats_reject_add` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `stats_cancel_add` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `stats_snapshot` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Why there is no `exchange_id` dependency despite the source design naming one:
[`../decisions/001_no_exchange_id_dependency.md`](../decisions/001_no_exchange_id_dependency.md).
