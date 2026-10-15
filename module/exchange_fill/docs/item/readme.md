# Item Doc Definition

### Scope

- **Purpose**: The full exposed surface this crate declares, as built vs. as the source proposal specified it.
- **Responsibility**: One consolidated table plus a comparison against `core_exchange.txt`.
- **In Scope**: `Trade`, `Event`, `EventKind`, `RejectReason`, `CancelCause`.
- **Out of Scope**: Build status prose for the whole crate (→ [`../../readme.md`](../../readme.md)); the central summary (→ [`../../../../docs/crate/013_exchange_fill.md`](../../../../docs/crate/013_exchange_fill.md)).

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `Trade` | struct | `{ taker: OrderId, taker_account: AccountId, taker_side: Side, maker: OrderId, maker_account: AccountId, price: Price, quantity: Quantity }` |
| `Trade::executed_price` | fn (assoc.) | `(maker: Price, _taker: Price) -> Price` |
| `RejectReason` | enum | `{ ZeroQuantity, NegativePrice, UnknownAccount, InsufficientFunds, ObligationUnrepresentable, ReservationUnrepresentable, Halted, RestsFull, LevelsFull, PostOnlyWouldTake, DuplicateClientId, AccountFull, PriceOffTick, QuantityOffLot, UnknownInstrument }` |
| `CancelCause` | enum | `{ Request, SelfMatch, TimeInForce }` |
| `Event` | struct | `{ sequence: Sequence, order: OrderId, account: AccountId, kind: EventKind }` |
| `EventKind` | enum | `{ OrderAccepted{side,price,quantity,reserved}, OrderRejected{reason}, Trade(Trade), OrderCancelled{cause,quantity,released} }` |

`Price`/`Quantity` are `exact_arith`'s.

### Differs from the proposal

The source design's exposed-item list (`core_exchange.txt:602-608`,
catalogued at
[`../../../../docs/exposed_item/013_exchange_fill_items.md`](../../../../docs/exposed_item/013_exchange_fill_items.md))
proposes `Fill { instrument, maker, taker, price, qty, maker_order, taker_order }`,
`Reject { order, reason }`, `CancelAck { order, qty_left }`, `fill_notional`,
`FillError { ZeroQty }` and a 9-variant `RejectReason`.

- **`Trade`, not `Fill`** — with account ids and `taker_side`, and no
  `instrument`: no instrument travels on the event stream at all.
- **No `Reject`/`CancelAck` structs** — `EventKind::OrderRejected` and
  `EventKind::OrderCancelled`, with the order and account on the `Event`.
- **No `fill_notional`** — `exchange_types::notional( trade.price, trade.quantity )`.
- **No `FillError`** — a zero quantity is `RejectReason::ZeroQuantity`, refused
  at submission.
- **`RejectReason` names none of the proposed nine.** Each of those belongs to
  the crate that detects it, so `docs/reject_reason/` stays central rather than
  moving here.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:428-433` | Crate 13 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:602-608` | Crate `exchange_fill`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
