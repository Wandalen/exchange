# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Sequence` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Sequence::ZERO` | const | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `seq_next` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Why there is no `seq_cmp` or `SeqError`: [`../decisions/readme.md`](../decisions/readme.md).
