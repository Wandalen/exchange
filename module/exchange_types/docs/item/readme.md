# item

The exposed surface of `exchange_types`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice; see this
family's own `docs/research/` for why a fuller `item_des.rulebook.md`
per-declaration split is deferred).

### Scope

- **Purpose**: What this crate actually exports, and how that surface has shrunk as types extracted to their own dedicated crates.
- **Responsibility**: One table, every exposed item (declared or re-exported), with a short note on provenance.
- **In Scope**: This crate's own public surface, including what it re-exports.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above); the full shape of a re-exported item beyond a brief gist — that belongs to the owning crate's own `docs/item/`.

### Overview Table

| Item | Kind | Signature | Owner |
|------|------|-----------|-------|
| `Price` | type alias | `= Money` | here |
| `Amount` | type alias | `= Money` | here |
| `TypeError` | enum | `{ NotionalOutOfRange, NotionalInexact }` | here |
| `notional` | fn | `(Price, Quantity) -> Result<Amount, TypeError>` | here |
| `obligation` | fn | `(&Order) -> Result<Obligation, TypeError>` | here |
| `Trade` | struct (re-export) | `{ taker, taker_account, taker_side, maker, maker_account, price, quantity }` | [`exchange_fill`](../../../exchange_fill/src/lib.rs) |
| `Event` | struct (re-export) | `{ sequence, order, account, kind }` | [`exchange_fill`](../../../exchange_fill/src/lib.rs) |
| `EventKind` | enum (re-export) | `{ OrderAccepted{side,price,quantity,reserved}, OrderRejected{reason}, Trade(Trade), OrderCancelled{cause,quantity,released} }` | [`exchange_fill`](../../../exchange_fill/src/lib.rs) |
| `RejectReason` | enum (re-export) | `{ ZeroQuantity, NegativePrice, UnknownAccount, InsufficientFunds, ObligationUnrepresentable, ReservationUnrepresentable }` | [`exchange_fill`](../../../exchange_fill/src/lib.rs) |
| `CancelCause` | enum (re-export) | `{ Request, SelfMatch }` | [`exchange_fill`](../../../exchange_fill/src/lib.rs) |
| `AccountId` | struct (re-export) | `(pub u64)` | [`exchange_id`](../../../exchange_id/docs/item/readme.md) |
| `OrderId` | struct (re-export) | `(pub u64)` | [`exchange_id`](../../../exchange_id/docs/item/readme.md) |
| `Order` | struct (re-export) | see owning crate | [`exchange_order`](../../../exchange_order/docs/item/readme.md) |
| `Obligation` | enum (re-export) | see owning crate | [`exchange_order`](../../../exchange_order/docs/item/readme.md) |
| `Sequence` | struct (re-export) | see owning crate | [`exchange_seq`](../../../exchange_seq/docs/item/readme.md) |
| `Side` | enum (re-export) | `{ Buy, Sell }` | [`exchange_side`](../../../exchange_side/docs/item/readme.md) |

`exchange_fill`'s own `docs/item/readme.md` does not exist yet (its
redistribution pass is pending) — the five rows above pointing at
`src/lib.rs` directly will gain a proper `docs/item/` pointer once that
pass lands; the signatures given here were read directly from
`exchange_fill/src/lib.rs` to stay accurate in the meantime.

### Not part of the 23-crate proposal

Unlike this family's other crates, `exchange_types` predates the 23-crate
proposal — it is one of the family's original 4 real crates, so there is
no `docs/crate/`, `docs/exposed_item/`, or `docs/crate_responsibility/`
central entry for it to compare against or thin. Its own fidelity concern
is internal: whether this file and the crate's own
[`readme.md`](../../readme.md) still describe the *current* residual
surface, not a comparison to an external spec. The crate's own module doc
([`src/lib.rs`](../../src/lib.rs)) and readme's "Extraction — shrinking,
not static" section are the authoritative history of what moved out and
when; not repeated here.
