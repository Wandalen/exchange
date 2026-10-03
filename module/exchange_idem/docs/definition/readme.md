# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here (`OrderId` is imported from `exchange_id`, not declared here).

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `IdSet` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `IdemError` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `idem_seen` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `idem_insert` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `idem_remove` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

No `docs/decisions/` — this crate's built surface matches the source
proposal exactly; see [`../item/readme.md`](../item/readme.md)'s own
"Matches the proposal" section.
