# Item Doc Definition

### Scope

- **Purpose**: The full exposed surface this crate declares, as built vs. as the source proposal specified it.
- **Responsibility**: One consolidated table plus a verified comparison against `core_exchange.txt`.
- **In Scope**: `Amount`, `Order`, `Obligation`.
- **Out of Scope**: Build status prose for the whole crate (→ [`../../readme.md`](../../readme.md)); the central summary (→ [`../../../../docs/crate/006_exchange_order.md`](../../../../docs/crate/006_exchange_order.md)).

### Overview Table

| Item | Signature | Purpose |
|------|-----------|---------|
| `Amount` | `pub type Amount = Money` | Currency amount — a total, not a per-unit `Price` |
| `Order` | `pub struct Order { id: OrderId, instrument: InstrumentId, account: AccountId, side: Side, price: Price, quantity: Quantity, tif: Tif, client: Option<ClientOrderId> }` | One submitted limit order, every field `pub` |
| `Obligation` | `pub enum Obligation { Cash(Amount), Asset(Quantity) }` | What an order commits until it fills or cancels |

### Differs from the proposal

The source design (`core_exchange.txt:386-391` and `:561-566`, catalogued at
[`../../../../docs/exposed_item/006_exchange_order_items.md`](../../../../docs/exposed_item/006_exchange_order_items.md))
specifies `Order { id, instrument, account, side, price, qty, tif, seq }` with
`order_new`, `order_qty_set`, `order_qty_left`, `order_is_empty`, and
`OrderError { ZeroQty, BadId }`. The real build:

- Has every proposed field except `seq` — `Sequence` is stamped on `Event`
  and on a resting order's `arrival`.
- Adds `client : Option< ClientOrderId >`, the submitter's own id, which
  `exchange_core` refuses to see twice from one account.
- Builds none of the five functions and types — see
  [`../decisions/001_no_order_mutation_or_error.md`](../decisions/001_no_order_mutation_or_error.md).
- Adds `Obligation` and its `Amount` alias — never assigned a crate by the
  proposal, and order-shaped.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:386-391` | Crate 6 in the source's Prompt 2 answer for workstream 002 |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:561-566` | Crate `exchange_order`'s exposed-item list in the source's Prompt 3 answer |
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
