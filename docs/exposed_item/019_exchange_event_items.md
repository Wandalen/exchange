# Exposed Item: exchange_event

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_event`.
- **Responsibility**: Draining fills, rejects, and cancel-acks for workstream 010.

**Design status**: Built — `event_push`/`event_drain`/`event_len`/`event_clear`
on a plain `Vec<Event>`; no `EventDrain` and no `EventError { Full }`, since
nothing bounds the stream. The proposed `Event { Fill, Reject, Cancel }` is
`exchange_fill`'s `Event { sequence, order, account, kind }` around a
four-variant `EventKind`, re-exported here. Built-vs-proposed comparison:
[`../../module/exchange_event/docs/item/readme.md`](../../module/exchange_event/docs/item/readme.md).

### Statement

Prompt 3 specifies a bounded, push/drain-style event queue with a capacity error.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:635-639` | Crate `exchange_event`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
