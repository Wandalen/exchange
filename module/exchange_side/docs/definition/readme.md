# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Side` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Side::opposite` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `side_opposite` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `side_is_bid` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `side_is_ask` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Why the variants are `Buy`/`Sell`, not `Bid`/`Ask`: [`../decisions/readme.md`](../decisions/readme.md).
