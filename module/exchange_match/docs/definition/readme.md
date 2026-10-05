# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface, including `SelfMatchPolicy`, re-exported here from `exchange_stp` rather than declared.
- **Out of Scope**: Items declared elsewhere and merely used here (`Book` from `exchange_book`; `tif_requires_full` from `exchange_tif`; `AccountId`/`OrderId` from `exchange_id`; `Order` from `exchange_order`; `Side` from `exchange_side`; `Trade` from `exchange_fill`; `Price`/`notional` from `exchange_types`; `Quantity`/`KindError` from `exact_arith`).

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Crossing` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Crossing::is_complete` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Crossing::filled` | method | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `SelfMatchCancellation` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `MatchError` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `SelfMatchPolicy` | enum (re-export) | `../../../exchange_stp/src/lib.rs`, re-exported at `src/lib.rs` | [`../item/readme.md`](../item/readme.md), full definition at [`../../../exchange_stp/docs/item/readme.md`](../../../exchange_stp/docs/item/readme.md) |
| `cross` | function | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

No decisions collection — the real build's divergences from the proposal
(one `cross` entry point instead of four phase functions, a 2-variant
`MatchError`, no `MatchOut::rejects`) are each already argued in
`src/lib.rs`'s own module doc; see [`../item/readme.md`](../item/readme.md)
for the comparison and its citations.
