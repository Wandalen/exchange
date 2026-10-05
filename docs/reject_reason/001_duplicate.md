# Reject Reason: Duplicate

### Scope

- **Purpose**: Refuse a retry of an `OrderId` that has already been placed on this book.
- **Responsibility**: One of the nine reject-reason values.
- **In Scope**: The idempotency check `exchange_idem` is proposed to own.
- **Out of Scope**: Any other admission check (capacity, halt, snap, …).

**Design status**: Not implemented as named — the real `RejectReason` enum (`exchange_fill/src/lib.rs:108`, extracted out of `exchange_types` this session — `exchange_types` re-exported it for a time, now retired) has a completely different variant set: `ZeroQuantity`, `NegativePrice`, `UnknownAccount`, `InsufficientFunds`, `ObligationUnrepresentable`, `ReservationUnrepresentable`. None of these 9 proposed variants exist by this name in the real code.

### Statement

`Duplicate` is the reason a resubmitted `OrderId` is refused rather than silently inserted a second time — the proposal's own pitfall tape names "retry inserts a second rest" as the failure this reason exists to prevent. No idempotency check exists in the real book today, so this path cannot currently be reached or tested.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1177-1186` | Prompt 9's `reject_reason` instance list, item 1 of 9 |
