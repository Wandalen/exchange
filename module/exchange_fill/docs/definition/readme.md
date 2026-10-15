# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here (`AccountId`/`OrderId` from `exchange_id`, `Obligation` from `exchange_order`, `Sequence` from `exchange_seq`, `Side` from `exchange_side` — none re-exported, only consumed by field types below).

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Trade` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Trade::executed_price` | fn (assoc.) | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `RejectReason` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `CancelCause` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Event` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `EventKind` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
