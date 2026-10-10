# Exposed Item: exchange_fill

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_fill`.
- **Responsibility**: Fill, reject, and cancel-ack — the three outcomes a submission can report.

**Design status**: Built as `Trade`, `Event`, `EventKind`, `RejectReason`,
`CancelCause` — no `Fill`, `Reject`, `CancelAck`, `fill_notional` or
`FillError`; `RejectReason` names none of the proposed nine. Built-vs-proposed
comparison: [`../../module/exchange_fill/docs/item/readme.md`](../../module/exchange_fill/docs/item/readme.md).

### Statement

Prompt 3 specifies a `Fill`/`Reject`/`CancelAck` trio with their own error type.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:602-608` | Crate `exchange_fill`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
