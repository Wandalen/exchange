# Reject Reason: Snap

### Scope

- **Purpose**: Refuse a price or quantity that does not land exactly on its instrument's tick or lot grid.
- **Responsibility**: One of the nine reject-reason values.
- **In Scope**: The tick/lot admission check `exchange_spec` is proposed to own, calling into workstream 006's snap functions.
- **Out of Scope**: Any rounding or snapping performed silently — a mismatch here is a reject, never a quiet correction.

**Design status**: Not implemented as named — the real `RejectReason` enum (`exchange_fill/src/lib.rs:108`, extracted out of `exchange_types` this session — `exchange_types` now only re-exports it) has a completely different variant set: `ZeroQuantity`, `NegativePrice`, `UnknownAccount`, `InsufficientFunds`, `ObligationUnrepresentable`, `ReservationUnrepresentable`. None of these 9 proposed variants exist by this name in the real code. No `InstrumentSpec`, tick, or lot concept exists at all in the real crates.

### Statement

`Snap` is the reason an order is refused when its price or quantity does not already sit on the instrument's tick or lot grid — the proposal's own pitfall tape warns against "reimplementing tick snap instead of calling 006," meaning this check is meant to call workstream 006's existing snap functions rather than re-derive the rule locally.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1177-1186` | Prompt 9's `reject_reason` instance list, item 4 of 9 |
