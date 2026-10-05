# Reject Reason: Halted

### Scope

- **Purpose**: Refuse new placement while an instrument's matching is frozen.
- **Responsibility**: One of the nine reject-reason values.
- **In Scope**: The halt check `exchange_halt` is proposed to own.
- **Out of Scope**: Cancelling or affecting orders already resting — halt freezes matching, it does not clear the book.

**Design status**: Not implemented as named — the real `RejectReason` enum (`exchange_fill/src/lib.rs:108`, extracted out of `exchange_types` this session — `exchange_types` re-exported it for a time, now retired) has a completely different variant set: `ZeroQuantity`, `NegativePrice`, `UnknownAccount`, `InsufficientFunds`, `ObligationUnrepresentable`, `ReservationUnrepresentable`. None of these 9 proposed variants exist by this name in the real code. No halt mechanism exists at all in the real crates.

### Statement

`Halted` is the reason a `place` call is refused while an instrument is frozen — a circuit-breaker or station-lockdown state in which rests already on the book remain, but nothing new may be admitted and nothing may cross, until `resume` lifts it.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1177-1186` | Prompt 9's `reject_reason` instance list, item 3 of 9 |
