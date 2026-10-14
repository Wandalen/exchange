# Neighbor Contract: 010 Implements EscrowPort

### Scope

- **Purpose**: State that the real balance ledger behind the escrow port is workstream 010's responsibility, not 002's.
- **Responsibility**: 010 implements `EscrowPort` (hold_try/hold_release/hold_commit) and consumes `Fill` events to settle.
- **In Scope**: The contract's two directions — 010 implements the trait 002 defines, and 010 consumes the events 002 emits.
- **Out of Scope**: 002 writing any real balance-holding logic itself.

**Design status**: Not held as drawn — see `../boundary/002_out.md`'s Design status: the real `exchange_escrow` crate implements real balance-holding itself rather than leaving it to a future workstream 010 crate.

### Statement

The intended contract is a clean two-way handoff: 002 defines the `EscrowPort` trait and calls it without knowing its implementation, and 010 both implements that trait and consumes 002's `Fill` events to perform settlement. In the real build, this handoff never happens — 002 took on 010's half of the contract itself before 010 existed, which is the single most significant boundary divergence the comparison between this proposal and the real build surfaces.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1090` | Third bullet of Prompt 9's `neighbor_contract` list |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1251-1253` | Workstream 010's own charter, confirming it is meant to implement 002's EscrowPort and settle Fill events |
