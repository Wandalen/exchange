# Exposed Item: exchange_rest

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_rest`.
- **Responsibility**: Placing, cancelling, and replacing a resting order.

**Design status**: Partially folded into the real `exchange_book` and `exchange_core` crates:
- `rest_place` ≈ `Book::insert` (`exchange_book`) orchestrated by `Exchange::submit`'s step 5 (`exchange_core/src/lib.rs:348-360`).
- `rest_cancel` ≈ `Book::cancel` (`exchange_book`) orchestrated by `Exchange::cancel` (`exchange_core/src/lib.rs:391-407`), which also releases the escrow hold — a step the proposal's `exchange_rest` crate (dependent only on `book, idem, cap, escrow, spec`) implies but does not spell out as one atomic caller-facing operation the way the real `Exchange::cancel` does.
- **`rest_replace` does not exist anywhere** — there is no atomic cancel-and-reinsert operation; a caller wanting to replace an order would need to call `cancel` then `submit` as two separate, non-atomic steps, losing whatever ordering/price-priority guarantee a dedicated `replace` might offer.
- `RestError { Duplicate, Missing, Full, Halted, Escrow, Snap }` does not exist as its own type — the real equivalents are spread across `ExchangeError` (`Rejected`, `Escrow`, `Matching`, `NotResting`) and `RejectReason`, with `Duplicate`, `Full`, `Halted`, and `Snap` all absent (see `../reject_reason/`).

### Statement

Prompt 3 specifies a `rest_place`/`rest_cancel`/`rest_replace` trio with a dedicated error type. The real build has the first two, spread across `exchange_book` and `exchange_core`, but no `replace` operation at all — this is a concrete, verified feature gap (see `../feature/025_book_replace.md`).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:615-617` | Crate `exchange_rest`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
