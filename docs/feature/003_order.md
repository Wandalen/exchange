# Feature: Order

### Scope

- **Purpose**: One record for a resting or taker order.
- **Responsibility**: Carry id, instrument, account, side, price, qty, tif, and sequence in one record.

**Design status**: Partially held — the real `Order` (`exchange_types/src/lib.rs:99-111`) has id/account/side/price/quantity but no `tif` field and no `instrument` field, both named in this feature's scope.

### Statement

An order is one record naming who submitted it, which instrument, which side, at what price and quantity, under what time-in-force, and at what sequence — not a loose tuple assembled ad hoc at each call site.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:311` | Feature 3 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1034` | Feature 3's English title, Prompt 9 |
