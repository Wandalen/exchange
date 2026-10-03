# Crate: exchange_rest

### Scope

- **Purpose**: Rest, cancel, replace.
- **Responsibility**: Own the three ways an order enters or leaves the book outside of matching.
- **In Scope**: Rest, cancel, replace.
- **Out of Scope**: Crossing — this crate does not cross.

**Design status**: Partially folded into the real `exchange_book` crate — `insert`/`cancel` exist; no `replace` method exists anywhere (verified via grep of `module/exchange_book/src/lib.rs` public functions).

### Statement

Without this crate, there is no way into the book except through a match. It closes hard problems 9 (cancel and replace) and 15 (idempotent order ids), and features 8 (`book_rest`, `book_cancel`) and 25 (`book_replace`). It depends on `exchange_book`, `exchange_idem`, `exchange_cap`, `exchange_escrow`, and `exchange_spec`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:440-445` | Crate 15 in the source's Prompt 2 answer for workstream 002 |
