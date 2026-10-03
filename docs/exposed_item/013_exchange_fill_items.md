# Exposed Item: exchange_fill

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_fill`.
- **Responsibility**: Fill, reject, and cancel-ack — the three outcomes a submission can report.

**Design status**: Built, in its own `exchange_fill` crate (extracted out
of `exchange_types` this session — the citations into
`exchange_types/src/lib.rs` this file previously carried are now stale;
corrected below to point at this crate's own source). Full comparison
moved to [`../../module/exchange_fill/docs/item/readme.md`](../../module/exchange_fill/docs/item/readme.md)
("Differs from the proposal"); summary:
- Proposed: `Fill { instrument, maker, taker, price, qty, maker_order, taker_order }`, `RejectReason {9 variants}`, `Reject { order, reason }`, `CancelAck { order, qty_left }`, `fill_notional`, `FillError { ZeroQty }`.
- Real: `Trade { taker, taker_account, taker_side, maker, maker_account, price, quantity }` (no type literally named `Fill` — see `exchange_fill/src/lib.rs:3-11`'s own module doc for why). No `instrument` field (no multi-instrument support).
- `RejectReason` is real but has a completely different 6-variant set — none of the proposed 9 exist by name (see `../reject_reason/` for the full per-reason accounting, and the crate's own item doc for why that collection does not redistribute here).
- `Reject`/`CancelAck` as standalone types do not exist — folded into `EventKind::OrderRejected`/`EventKind::OrderCancelled` (`exchange_fill/src/lib.rs:192-226`).
- `fill_notional` does not exist here — `exchange_types::notional` computes it.
- `FillError { ZeroQty }` does not exist — zero-quantity rejection is `RejectReason::ZeroQuantity`, not a construction-step error.

### Statement

Prompt 3 specifies a `Fill`/`Reject`/`CancelAck` trio with their own error type. The real build has no type named `Fill` at all — `Trade` carries that role — and folds rejection/cancellation into the richer `EventKind` enum instead of two standalone structs.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:602-608` | Crate `exchange_fill`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
