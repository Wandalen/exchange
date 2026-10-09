# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface — the items `src/lib.rs` defines, not the ones it merely re-exports.
- **Out of Scope**: `CancelCause`, `Event`, `EventKind`, `RejectReason`, `Trade` (re-exported from `exchange_fill`), `AccountId`, `OrderId` (from `exchange_id`), `Obligation`, `Order` (from `exchange_order`), `Sequence` (from `exchange_seq`), `Side` (from `exchange_side`) — all declared and documented in their owning crate, not here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Amount` | type alias | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `TypeError` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `notional` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `obligation` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
