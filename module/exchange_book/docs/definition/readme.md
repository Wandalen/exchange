# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here (`InstrumentId`/`OrderId`/`AccountId` from `exchange_id`; `Side` from `exchange_side`; `Quantity`/`Price` from `exact_arith`; `Level`/`LevelNode`/`level_*` from `exchange_level`).

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Resting` | type alias | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book` | struct (opaque) | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book::new` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book::insert` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book::cancel` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book::side` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book::best` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book::consume_best` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book::len` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book::is_empty` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book::iter` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book::rests_at` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book::level_count` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Book::account_rests` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

No decisions collection: each divergence from the proposal is argued in
`src/lib.rs`, where a change to it would be made.
