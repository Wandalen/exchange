# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `SelfMatchPolicy` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `stp_name` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Why there is no `Allow`, and why the other two are named differently:
[`../decisions/readme.md`](../decisions/readme.md).
