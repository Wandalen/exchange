# Escrow Port: hold_commit

### Scope

- **Purpose**: Convert a lock into a real transfer when a fill actually happens.
- **Responsibility**: One of the three `EscrowPort` trait methods.
- **In Scope**: Consuming a hold into the settlement a `Fill` describes.
- **Out of Scope**: Returning a hold unconsumed — that is `hold_release`'s job.

**Design status**: Named differently in the real build — `exchange_escrow::Escrow` (`module/exchange_escrow/src/lib.rs`) exposes `settle()` rather than `hold_commit`, taking a `Trade` directly rather than a bare hold identifier.

### Statement

`hold_commit` is the operation a fill triggers: the amount already locked by `hold_try` is consumed into the transfer the trade describes, rather than returned to available balance. The proposal's pitfall tape separately warns against both "commit and release of the same hold" and a "saturating add hiding an insolvent hold" — two distinct ways this single operation can go wrong silently.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1172-1175` | Prompt 9's `escrow_port` instance list, item 3 of 3 |
