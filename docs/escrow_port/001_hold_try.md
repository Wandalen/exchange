# Escrow Port: hold_try

### Scope

- **Purpose**: Lock funds or asset before an order is allowed to rest.
- **Responsibility**: One of the three `EscrowPort` trait methods.
- **In Scope**: Attempting a reservation and failing closed if funds are insufficient.
- **Out of Scope**: Releasing or committing an existing hold.

**Design status**: Named differently in the real build — `exchange_escrow::Escrow` (`module/exchange_escrow/src/lib.rs`) exposes `reserve()` rather than `hold_try`. Functionally analogous, but `Escrow` is a concrete struct with its own balance bookkeeping, not a trait named `EscrowPort` that a future workstream 010 crate was meant to implement.

### Statement

`hold_try` is the first of the three escrow-port operations: it attempts to lock the exact amount an order could cost, and the order is only allowed to rest or match if this succeeds. The proposal's own pitfall tape names "resting before `hold_try` succeeds" as a specific bug this ordering exists to prevent.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1172-1175` | Prompt 9's `escrow_port` instance list, item 1 of 3 |
