# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `RestRow` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `BookSnap` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `snap_take` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `snap_len` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Why there is no `exchange_order` dependency or `SnapError` despite the
source design naming both:
[`../decisions/readme.md`](../decisions/readme.md).
