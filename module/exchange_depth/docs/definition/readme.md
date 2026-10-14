# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here (`Book`/`Resting` from `exchange_book`; `InstrumentId` from `exchange_id`; `Side` from `exchange_side`; `Price`/`Quantity` from `exact_arith`).

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `LevelView` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Depth` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `DepthError` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `depth_top` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
