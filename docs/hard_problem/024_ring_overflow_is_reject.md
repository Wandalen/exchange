# Hard Problem: Ring Overflow Is Reject

### Scope

- **Purpose**: A full ring doesn't swallow the order.
- **Responsibility**: Turn a full ring into an explicit Reject for the order that didn't fit, never a silent drop.

### Statement

A full ring must not silently swallow an order — a silent drop means money that was already reserved in escrow vanishes while the player believes their order is resting. Ring overflow must surface as an explicit Reject.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:302-305` | Hard problem 24 in the source's Prompt 1 answer for workstream 002 |
