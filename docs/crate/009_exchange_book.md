# Crate: exchange_book

### Scope

- **Purpose**: Sorted bid and ask ladders for one instrument.
- **Responsibility**: Hold the resting orders of one instrument in price order.
- **In Scope**: Storage and lookup. No match, no escrow, no `ring_*` import.
- **Out of Scope**: Matching and escrow.

**Design status**: Built as the real `exchange_book` crate — `Book`, `Resting`, with `insert`/`cancel`/`best`/`consume_best`/`side`/`iter`. Single-instrument only (no `InstrumentId` anywhere in its API).

### Statement

Without this crate there is no market at all. It closes hard problems 1 (one book per instrument), 13 (depth), 14 (hot path), and 17 (capacity), and features 6 (book) and 22 (sorted price walk). It depends on `exchange_level`, `exchange_spec`, and `exchange_id`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:404-409` | Crate 9 in the source's Prompt 2 answer for workstream 002 |
