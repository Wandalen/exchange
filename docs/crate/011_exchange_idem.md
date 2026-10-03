# Crate: exchange_idem

### Scope

- **Purpose**: An `OrderId` seen once per book.
- **Responsibility**: Detect a repeated `OrderId` before it reaches the ladder.
- **In Scope**: The seen-id set itself.
- **Out of Scope**: The ladder — this crate is not it.

**Design status**: Built, in its own `exchange_idem` crate — matches the proposal exactly. Verified built-vs-proposed comparison: [`../../module/exchange_idem/docs/item/readme.md`](../../module/exchange_idem/docs/item/readme.md).

### Statement

Without idempotency, a retry doubles the rest instead of being recognized as the same order. This crate closes hard problem 15 (idempotent order ids) and feature 19 (unique `OrderId` per book). It depends on `exchange_id`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:416-421` | Crate 11 in the source's Prompt 2 answer for workstream 002 |
