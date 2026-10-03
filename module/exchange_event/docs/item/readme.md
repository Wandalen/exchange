# item

The exposed surface of `exchange_event`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice; see this
family's own `docs/research/` for why a fuller `item_des.rulebook.md`
per-declaration split is deferred).

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a short note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above).

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `Event` | struct (re-export) | `{ sequence, order, account, kind }` — from `exchange_types` |
| `EventKind` | enum (re-export) | `{ OrderAccepted{side,price,quantity,reserved}, OrderRejected{reason}, Trade(Trade), OrderCancelled{cause,quantity,released} }` — from `exchange_types` |
| `event_push` | fn | `(&mut Vec<Event>, Event)` |
| `event_drain` | fn | `(&mut Vec<Event>) -> Vec<Event>` |
| `event_len` | fn | `(&[Event]) -> usize` |
| `event_clear` | fn | `(&mut Vec<Event>)` |

### Differs from the proposal

The central catalog
([`../../../../docs/exposed_item/019_exchange_event_items.md`](../../../../docs/exposed_item/019_exchange_event_items.md))
already records the fold between Prompt 3's proposed
`Event { Fill, Reject, Cancel }` and the real, richer `Event`/`EventKind`
struct-plus-enum shape — not repeated here. What this crate adds on top of
that already-documented fold: all four plain-`Vec<Event>` functions the
proposal names for it — `event_push`/`event_drain`/`event_len`/
`event_clear` — none of which `exchange_types` or
`exchange_core::Exchange::events()` (a borrowed `&[Event]` only) provides.
No `EventDrain` type or `EventError` — both need a bounded-capacity queue
to have meaning, and no such type exists anywhere in the real build; see
[`../decisions/001_no_event_drain_type.md`](../decisions/001_no_event_drain_type.md)
and
[`../decisions/002_no_event_error.md`](../decisions/002_no_event_error.md).
