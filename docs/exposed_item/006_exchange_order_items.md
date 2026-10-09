# Exposed Item: exchange_order

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_order`.
- **Responsibility**: One order record, rest or taker, with its quantity-management helpers.

**Design status**: Built, without the helpers or `OrderError`, with `client` added. Built-vs-proposed comparison: [`../../module/exchange_order/docs/item/readme.md`](../../module/exchange_order/docs/item/readme.md).

### Statement

Prompt 3 specifies `Order { id, instrument, account, side, price, qty, tif, seq }` with constructor and quantity-mutation helpers plus its own `OrderError`. See the per-crate doc linked above for the current, verified comparison.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:561-565` | Crate `exchange_order`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
