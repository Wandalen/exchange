# item

The exposed surface of `exchange_fill`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice; see this
family's own `docs/research/` for why a fuller `item_des.rulebook.md`
per-declaration split is deferred).

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a short note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above).

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `Trade` | struct | `{ taker: OrderId, taker_account: AccountId, taker_side: Side, maker: OrderId, maker_account: AccountId, price: Price, quantity: Quantity }` |
| `Trade::executed_price` | fn (assoc.) | `(maker: Price, _taker: Price) -> Price` |
| `RejectReason` | enum | `{ ZeroQuantity, NegativePrice, UnknownAccount, InsufficientFunds, ObligationUnrepresentable, ReservationUnrepresentable, Halted, RestsFull, LevelsFull, PostOnlyWouldTake, DuplicateClientId }` — `RestsFull`/`LevelsFull` added for `exchange_cap`'s `exchange_core` wiring, `PostOnlyWouldTake` for `Tif::PostOnly`, `DuplicateClientId` for `ClientOrderId` |
| `CancelCause` | enum | `{ Request, SelfMatch }` |
| `Event` | struct | `{ sequence: Sequence, order: OrderId, account: AccountId, kind: EventKind }` |
| `EventKind` | enum | `{ OrderAccepted{side,price,quantity,reserved}, OrderRejected{reason}, Trade(Trade), OrderCancelled{cause,quantity,released} }` |

`Price`/`Quantity` here are `exact_arith::Price`/`exact_arith::Quantity` —
re-exports of `exact_kind::{Price,Quantity}` directly, not
`exchange_types::Price` (a separate alias for the identical underlying
`Decimal<MONEY_SCALE>` — see `exact_kind/src/lib.rs`'s own module doc,
"Money and Price are the same type today"). Interchangeable in practice,
not a divergence.

### Differs from the proposal

The source design's own exposed-item list
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:602-608`,
also catalogued centrally at
[`../../../../docs/exposed_item/013_exchange_fill_items.md`](../../../../docs/exposed_item/013_exchange_fill_items.md) —
whose own citations into `exchange_types/src/lib.rs` are now stale, see
below) proposes `Fill{instrument,maker,taker,price,qty,maker_order,taker_order}`,
`Reject{order,reason}`, `CancelAck{order,qty_left}`, `fill_notional`, and
`FillError{ZeroQty}`, plus a 9-variant `RejectReason`. None of that survives
by name:

- **No `Fill` type.** `Trade` carries the role — this crate's own module
  doc explains why ("the family's Contract says *trades out*... 'Fill'
  survives as a verb... never a second record of it"). No `instrument`
  field either (no multi-instrument support at this layer).
- **No `Reject`/`CancelAck` structs.** Both outcomes fold into the richer
  `EventKind::OrderRejected{reason}` / `EventKind::OrderCancelled{cause,
  quantity,released}` instead.
- **No `fill_notional` function.** `exchange_types::notional(price,
  quantity)` already computes this, more generally, and this crate's own
  `Trade` is what callers pass through it (see `exchange_conserve::conserve_assert`
  for exactly that call).
- **No `FillError{ZeroQty}`.** Zero-quantity rejection happens at
  `exchange_core::Exchange::submit` via `RejectReason::ZeroQuantity`, not at
  a `Fill`-construction step that no longer exists.
- **`RejectReason` is real but has a completely different 6-variant set** —
  none of the proposal's `Duplicate/Full/Halted/Snap/Stp/Fok/Escrow/Overflow/Unknown`
  exist by name. See "reject_reason/ is not this crate's to redistribute",
  below.
- **`taker_side` added to `Trade`**, not in the proposal — see this crate's
  own module doc ("`taker_side`, added to `Trade`") for why
  (`exchange_conserve` needs to classify a trade's sides with no submission
  context available).

### Central doc staleness found while redistributing

`docs/crate/013_exchange_fill.md` and `docs/exposed_item/013_exchange_fill_items.md`
both said "Folded into the real `exchange_types` crate" — stale since
`Trade`/`Event`/`EventKind`/`RejectReason`/`CancelCause` extracted out of
`exchange_types` into this crate, their own, earlier this session. The
exposed-item file's own citations into `exchange_types/src/lib.rs:22-30`
and `:260-295` are also now wrong — that file is 162 lines today and
none of this crate's types live there any more. Thinned/corrected as part
of this redistribution pass rather than left stale.

### `reject_reason/` is not this crate's to redistribute

The central `docs/reject_reason/` collection (9 instances) looks at first
glance like it belongs here, since `RejectReason` is this crate's own enum.
It does not: each of the 9 proposed variants names a **different** proposed
owning crate in its own "In Scope" field — `001_duplicate.md` →
`exchange_idem`, `002_full.md` → `exchange_cap`, `003_halted.md` →
`exchange_halt`, `004_snap.md` → `exchange_spec`, `008_overflow.md` →
the not-yet-existing `exchange_inbound`, and so on — none of them point at
this crate. The real, built `RejectReason` here has a disjoint 6-variant
set matching none of the 9 by name (see Overview Table, above). Redistributing
all 9 into this crate would misattribute idem/cap/halt/spec/stp/match/escrow
concerns to the one crate that merely holds the (unrelated) real enum.
Genuinely relevant to ≥2 crates — correctly stays central, unredistributed.
