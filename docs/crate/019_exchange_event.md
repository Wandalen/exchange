# Crate: exchange_event

### Scope

- **Purpose**: Drain fills, rejects, and cancel-acks for workstream 010.
- **Responsibility**: Own the one drain point consumers read from.
- **In Scope**: The drain itself. Not a ring — a second ring for fills is later, not now.
- **Out of Scope**: Being a ring.

**Design status**: Built — `event_push`/`event_drain`/`event_len`/`event_clear`
over `exchange_fill`'s `Event`/`EventKind`, re-exported; no `EventDrain` or
`EventError`. `Exchange::event_drain` calls it. Built-vs-proposed comparison:
[`../../module/exchange_event/docs/item/readme.md`](../../module/exchange_event/docs/item/readme.md).

### Statement

Without this crate, workstream 002 would end up writing wallets directly. It closes hard problem 12 (events, not wallets) and feature 26 (event drain for 010). It depends on `exchange_fill`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:464-469` | Crate 19 in the source's Prompt 2 answer for workstream 002 |
