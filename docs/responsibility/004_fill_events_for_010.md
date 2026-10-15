# Responsibility: Fill Events for 010

### Scope

- **Purpose**: State that settlement happens downstream of 002, driven by the events 002 emits.
- **Responsibility**: 002 emits Fill/Reject/CancelAck events; it does not itself move wallet balances.
- **In Scope**: The `Event`/`EventKind` stream and its drain.
- **Out of Scope**: Crediting or debiting any account — that is workstream 010's `settle_apply`, triggered by these events.

### Statement

002's output, beside the book state itself, is an event stream — Fill, Reject, CancelAck — that workstream 010 consumes to perform real settlement. This is the same "events, not wallets" principle as responsibility 003, applied to the output side rather than the input side: 002 never credits an account, it only reports what happened.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1001` | Fourth bullet of Prompt 9's `responsibility` list |
