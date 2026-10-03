# Feature: book_replace

### Scope

- **Purpose**: Change a resting order without a cancel/re-rest race.
- **Responsibility**: Replace an order's terms atomically — old gone, new live.

**Design status**: Not built as a dedicated operation — the real `exchange_book` has `insert`/`cancel` but no `replace` method (verified via grep of `module/exchange_book/src/lib.rs` public functions).

### Statement

`book_replace` changes a resting order's terms as one atomic step, so the old order is reliably gone and the new one reliably live — never a window where a plain cancel-then-rest could lose priority or leave neither in place.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:333` | Feature 25 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1056` | Feature 25's English title, Prompt 9 |
