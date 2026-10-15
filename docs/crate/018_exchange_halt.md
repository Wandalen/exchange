# Crate: exchange_halt

### Scope

- **Purpose**: Freeze the match; rests remain.
- **Responsibility**: Give the book an on/off switch for matching alone.
- **In Scope**: The halt flag itself.
- **Out of Scope**: Removing orders — halt never does that.

**Design status**: Built as proposed, without the `exchange_book` dependency; `exchange_core` refuses new orders on a halted instrument. Built-vs-proposed comparison: [`../../module/exchange_halt/docs/item/readme.md`](../../module/exchange_halt/docs/item/readme.md).

### Statement

Without a halt, there is no way to stop a market — no station lockdown, no circuit breaker. This crate closes hard problem 18 (halt) and feature 18 (`book_halt`, `book_resume`). It depends on `exchange_spec` and `exchange_book`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:458-463` | Crate 18 in the source's Prompt 2 answer for workstream 002 |
