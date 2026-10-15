# Item Doc Definition

### Scope

- **Purpose**: The full exposed surface this crate declares.
- **Responsibility**: One consolidated table.
- **In Scope**: `TypeError`, `notional`, `obligation`.
- **Out of Scope**: Build status prose for the whole crate (→ [`../../readme.md`](../../readme.md)).

### Overview Table

| Item | Signature | Purpose |
|------|-----------|---------|
| `TypeError` | `pub enum TypeError { NotionalOutOfRange, NotionalInexact }` | A notional the currency type cannot hold exactly |
| `notional` | `pub fn notional(price: Price, quantity: Quantity) -> Result<Money, TypeError>` | The exact currency value of `quantity` at `price` |
| `obligation` | `pub fn obligation(order: &Order) -> Result<Obligation, TypeError>` | What `order` must reserve to rest |

### Not part of the 23-crate proposal

`exchange_types` predates the proposal — it is one of the family's original
four crates — so there is no `docs/crate/` or `docs/exposed_item/` entry to
compare against. Everything it once declared or re-exported now lives in
`exchange_id`, `exchange_side`, `exchange_seq`, `exchange_order` and
`exchange_fill`.

### Sources

| File | Relationship |
|------|--------------|
| [`../../src/lib.rs`](../../src/lib.rs) | The real, built shape this table describes |
