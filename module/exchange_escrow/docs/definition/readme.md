# Definition Doc Definition

### Scope

- **Purpose**: Index every `pub` item this crate declares against the doc instance that explains it.
- **Responsibility**: One row per public declaration, pointing at its owning collection — never restating what that collection already says.
- **In Scope**: This crate's own `pub` surface.
- **Out of Scope**: Items declared elsewhere and merely used here (`exchange_types`, `exact_arith`).

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Conserved` | trait | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Holding<T>` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Account` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `EscrowError` | enum | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |
| `Escrow` | struct | `src/lib.rs` | [`../item/readme.md`](../item/readme.md) |

Why `Escrow` holds real account balances instead of being the thin port the
proposal named `EscrowPort`: [`../item/readme.md`](../item/readme.md)'s
"Differs from the proposal" section, and centrally
[`../../../../docs/decision/003_010_implements_escrow_and_consumes_fills.md`](../../../../docs/decision/003_010_implements_escrow_and_consumes_fills.md).
