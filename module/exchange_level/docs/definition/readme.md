# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here (`Order` is imported from `exchange_order`, not declared here; `Price`/`Quantity` from `exact_arith`; `OrderId` from `exchange_id`; `Sequence` from `exchange_seq`).

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `LevelNode` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Level` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `level_new` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `level_push` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `level_pop_front` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `level_remove` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `level_len` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `level_qty_sum` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `level_empty_is` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Why there is no `LevelError`: [`../decisions/001_no_level_error.md`](../decisions/001_no_level_error.md).
Why the nodes are not an intrusive list: [`../../readme.md`](../../readme.md).
