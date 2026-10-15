# item

The exposed surface of `exchange_event`, as built.

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a short note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Per-declaration files.

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `Event` | struct (re-export) | `{ sequence, order, account, kind }` — from `exchange_fill` |
| `EventKind` | enum (re-export) | `{ OrderAccepted{side,price,quantity,reserved}, OrderRejected{reason}, Trade(Trade), OrderCancelled{cause,quantity,released} }` — from `exchange_fill` |
| `event_push` | fn | `(&mut Vec<Event>, Event)` |
| `event_drain` | fn | `(&mut Vec<Event>) -> Vec<Event>` |
| `event_len` | fn | `(&[Event]) -> usize` |
| `event_clear` | fn | `(&mut Vec<Event>)` |

### Differs from the proposal

The four functions are built as the source design names them
(`core_exchange.txt:636-639`). Not built: the `EventDrain` container and
`EventError { Full }` — see
[`../decisions/001_no_event_drain_type.md`](../decisions/001_no_event_drain_type.md)
and [`../decisions/002_no_event_error.md`](../decisions/002_no_event_error.md).
The proposed `Event { Fill, Reject, Cancel }` became `exchange_fill`'s
`Event`/`EventKind` — see
[`../../../../docs/exposed_item/019_exchange_event_items.md`](../../../../docs/exposed_item/019_exchange_event_items.md).
