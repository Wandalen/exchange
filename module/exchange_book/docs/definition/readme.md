# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here (`InstrumentId` from `exchange_id`; `OrderId`/`Side` from `exchange_types`; `Quantity`/`Price` from `exact_arith`; `Level`/`level_*` from `exchange_level`, which `Resting` type-aliases and `insert`/`cancel` call into).

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

No decisions collection — the real build's divergences from the proposal
(instrument-keying, collapsed per-side functions, no dedicated error type)
are each already explained where they arise, in `src/lib.rs`'s own module
and method doc comments; see [`../item/readme.md`](../item/readme.md) for
the comparison and its citations.
