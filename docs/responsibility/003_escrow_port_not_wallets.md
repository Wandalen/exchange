# Responsibility: Escrow Port, Not Wallets

### Scope

- **Purpose**: Draw the boundary between 002 (which reserves funds for an order) and 010 (which actually owns balances).
- **Responsibility**: 002 defines and calls an escrow port; it does not implement wallet balances itself.
- **In Scope**: The `EscrowPort` trait and its three operations (hold_try, hold_release, hold_commit).
- **Out of Scope**: The real balance ledger — that belongs to workstream 010.

**Design status**: Not held as drawn in the real build — see `../boundary/002_out.md`'s Design status. The real `exchange_escrow` crate holds actual account balances itself, rather than being a thin port workstream 010 implements.

### Statement

This responsibility states a boundary, not just a feature: 002 is supposed to call an escrow port (hold/release/commit) without itself knowing how balances are stored or represented — that is workstream 010's job once it exists. The real build, built before 010 exists, collapsed this boundary by implementing real balance-holding directly inside `exchange_escrow`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1000` | Third bullet of Prompt 9's `responsibility` list |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:422-426` | `exchange_escrow`'s crate definition in Prompt 2: "Межа: немає балансів; 010 реалізує порт" (no balances; 010 implements the port) |
