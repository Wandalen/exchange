# Hard Problem: Idempotent Order Id

### Scope

- **Purpose**: A retry is the same order.
- **Responsibility**: Treat a retried submission of the same OrderId as the same order, not a second one.

**Design status**: Not addressed — no duplicate-`OrderId` detection exists; `RejectReason` has no `Duplicate` variant.

### Statement

The host and the UI can both resend the same submission, so a retried OrderId must resolve to the same order rather than resting twice.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:257-260` | Hard problem 15 in the source's Prompt 1 answer for workstream 002 |
