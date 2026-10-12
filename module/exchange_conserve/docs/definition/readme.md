# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: `Trade`/`Side`/`TypeError`, used here but declared and documented in `exchange_fill`/`exchange_side`/`exchange_types`.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `ConserveError` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `conserve_assert` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Why this is not `exchange_escrow::total_cash`/`total_asset`:
[`../../src/lib.rs`](../../src/lib.rs)'s module doc.
