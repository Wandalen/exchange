# Feature: EscrowPort

### Scope

- **Purpose**: A trait this workstream calls, that 010 implements.
- **Responsibility**: Expose hold_try, hold_release, hold_commit as the only lock mechanism.

### Statement

`EscrowPort` is the trait boundary between this workstream and real balances: `hold_try` reserves, `hold_release` returns a reservation on cancel, `hold_commit` consumes it on a fill — the book itself never touches a wallet directly.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:319` | Feature 11 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1042` | Feature 11's English title, Prompt 9 |
