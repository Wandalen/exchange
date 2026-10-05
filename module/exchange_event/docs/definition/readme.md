# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here (`Event`/`EventKind` are re-exported, not declared here — see `exchange_fill`'s own `docs/` for their definition).

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `event_push` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `event_drain` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `event_len` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `event_clear` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Why there is no `EventDrain` type or `EventError` despite the source design
naming both: [`../decisions/readme.md`](../decisions/readme.md).
