# Reject Reason: Escrow

### Scope

- **Purpose**: Name a failed `hold_try` as its own reason, distinct from book-level conditions.
- **Responsibility**: One of the nine reject-reason values.
- **In Scope**: Admission refused because the account could not lock the required funds or asset.
- **Out of Scope**: Any book-side condition (capacity, halt, duplicate, snap).

**Design status**: Partially held — the real `RejectReason` enum (`exchange_fill/src/lib.rs:108`, extracted out of `exchange_types` this session — `exchange_types` now only re-exports it) does carry this concept, just under different, more specific names: `InsufficientFunds`, `ObligationUnrepresentable`, and `ReservationUnrepresentable` each cover a distinct way an escrow reservation can fail, rather than one bare `Escrow` catch-all.

### Statement

`Escrow` would mark a reject caused by the escrow port refusing to lock funds — the proposal's pitfall tape's "resting before `hold_try` succeeds" bug is exactly what this reason exists to make visible instead of silent. The real build's three-way split (`InsufficientFunds`/`ObligationUnrepresentable`/`ReservationUnrepresentable`) is arguably more precise than this single proposed variant, distinguishing "genuinely insufficient" from two different representable-ceiling failures the proposal does not name at all.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1177-1186` | Prompt 9's `reject_reason` instance list, item 7 of 9 |
