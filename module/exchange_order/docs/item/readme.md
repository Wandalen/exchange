# Item Doc Definition

### Scope

- **Purpose**: The full exposed surface this crate declares, as built vs. as the source proposal specified it.
- **Responsibility**: One consolidated table plus a verified comparison against `core_exchange.txt`.
- **In Scope**: `Amount`, `Order`, `Obligation`.
- **Out of Scope**: Build status prose for the whole crate (→ [`../../readme.md`](../../readme.md)); the superseded central summary (→ [`../../../../docs/crate/006_exchange_order.md`](../../../../docs/crate/006_exchange_order.md), [`../../../../docs/exposed_item/006_exchange_order_items.md`](../../../../docs/exposed_item/006_exchange_order_items.md)).

### Overview Table

| Item | Signature | Purpose |
|------|-----------|---------|
| `Amount` | `pub type Amount = Price` | Currency amount — same type as `Price`, read as a total rather than a per-unit rate |
| `Order` | `pub struct Order { id: OrderId, instrument: InstrumentId, account: AccountId, side: Side, price: Price, quantity: Quantity, tif: Tif }` | One submitted limit order, every field `pub` |
| `Obligation` | `pub enum Obligation { Cash(Amount), Asset(Quantity) }` | What an order commits until it fills or cancels |

### Differs from the proposal

Verified directly against `core_exchange.txt:386-391` (crate 6, Prompt 2) and
`core_exchange.txt:561-566` (exposed-item list, Prompt 3) — not against the
central `docs/crate/006_exchange_order.md`/`docs/exposed_item/006_exchange_order_items.md`
summaries, which describe a now-superseded state (`Order` folded into
`exchange_types`, missing `instrument`/`tif`) and are thinned to point here.

The proposal specifies `Order { id, instrument, account, side, price, qty, tif, seq }`
with `order_new`, `order_qty_set`, `order_qty_left`, `order_is_empty`, and a
dedicated `OrderError { ZeroQty, BadId }`. The real build:

- Has every field the proposal names on `Order` **except** `seq` — `Sequence`
  is attached to `Event`, not to `Order` (see `exchange_seq`'s own readme).
  `instrument` and `tif` — the two fields missing when this crate was still
  folded into `exchange_types` — are both present now that `Order` has its
  own crate.
- Builds no `order_new` — every field stays `pub`, so a plain struct literal
  is the only construction path, with no second fallible path to keep in
  sync with it.
- Builds no `order_qty_set`/`order_qty_left`/`order_is_empty` — `Order.quantity`
  is immutable for the order's lifetime; the shrinking remainder lives on
  `exchange_book::Resting`/`exchange_level::LevelNode` once the order rests,
  since only a resting order has a remainder to track.
- Builds no `OrderError` — a zero-quantity order is refused at
  `exchange_core::Exchange::submit`'s boundary via `RejectReason::ZeroQuantity`,
  not at construction time.
- Adds `Obligation` (and its `Amount` alias) — not named in crate 6's own
  exposed-item list, but assigned here because the family's dependency tree
  never gave `Obligation` a crate of its own and what an order commits is
  order-shaped.

Full reasoning for each omission: [`../decisions/readme.md`](../decisions/readme.md).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:386-391` | Crate 6 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:561-566` | Crate `exchange_order`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
