# Hard Problem: Events, Not Wallets

### Scope

- **Purpose**: The output is Fill, Reject, CancelAck.
- **Responsibility**: Emit Fill/Reject/CancelAck events and leave all balance-keeping to workstream 010.

### Statement

Balances belong to workstream 010; this workstream's only output is the Fill/Reject/CancelAck event stream. Without that split, there would be two sources of truth for the same money.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:242-245` | Hard problem 12 in the source's Prompt 1 answer for workstream 002 |
