# Reject Reason: Full

### Scope

- **Purpose**: Refuse a rest once a book's capacity limit is reached, rather than growing unbounded or dropping silently.
- **Responsibility**: One of the nine reject-reason values.
- **In Scope**: The capacity check `exchange_cap` is proposed to own.
- **Out of Scope**: Any check unrelated to rest/level count.

**Design status**: Not implemented as named — the real `RejectReason` enum (`exchange_fill/src/lib.rs:108`, extracted out of `exchange_types` this session — `exchange_types` re-exported it for a time, now retired) has a completely different variant set: `ZeroQuantity`, `NegativePrice`, `UnknownAccount`, `InsufficientFunds`, `ObligationUnrepresentable`, `ReservationUnrepresentable`. None of these 9 proposed variants exist by this name in the real code.

### Statement

`Full` is the explicit, loud error a book returns once it hits its configured rest or level cap — the proposal's own pitfall tape calls out "Full drops the order and returns ok" as the specific bug this reason exists to prevent: capacity exhaustion must be visible to the caller, never a quiet no-op. No capacity limit exists in the real book today.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1177-1186` | Prompt 9's `reject_reason` instance list, item 2 of 9 |
