# Exposed Item: exchange_event

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_event`.
- **Responsibility**: Draining fills, rejects, and cancel-acks for workstream 010.

**Design status**: `Event`/`EventKind` stay in `exchange_types`, in a richer shape than proposed; `event_push`/`event_drain`/`event_len`/`event_clear` are now built, in their own `exchange_event` crate — this summary predates that crate's addition and understates the real surface. Verified built-vs-proposed comparison: [`../../module/exchange_event/docs/item/readme.md`](../../module/exchange_event/docs/item/readme.md).
- Proposed: `Event { Fill, Reject, Cancel }` (an enum of the three outcomes directly), `EventDrain`, `event_push`/`event_drain`/`event_len`/`event_clear`, `EventError { Full }`.
- Real: `Event { sequence, order, account, kind }` is a **struct** wrapping a separate `EventKind` enum (`OrderAccepted`, `OrderRejected`, `Trade`, `OrderCancelled`) — four variants, richer payloads, and a `sequence`/`order`/`account` envelope common to all of them (`exchange_types/src/lib.rs:238-295`).
- All four functions (`event_push`/`event_drain`/`event_len`/`event_clear`) exist in `exchange_event`, operating on `&mut Vec<Event>` directly — no standalone `EventDrain` type and no `EventError { Full }`, since nothing here has a bounded capacity to exceed.
- Notably, there is deliberately **no** `OrderFilled`/dedicated-fill-event kind (see the crate's own module doc, `exchange_types/src/lib.rs:254-259`): fill state is derived from summing `EventKind::Trade` events rather than recorded a second time.

### Statement

Prompt 3 specifies a bounded, push/drain-style event queue with a capacity error. The real event stream is an unbounded `Vec<Event>` — mutated directly through `exchange_event`'s own `event_push`/`event_drain`/`event_len`/`event_clear`, not only read via `exchange_core::Exchange::events()`'s slice — with a four-variant `EventKind` richer than the proposed three-way `Event` enum.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:635-639` | Crate `exchange_event`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
