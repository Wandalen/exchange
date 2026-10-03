# Pitfall: A second fill ring in 002 before 010 asks for it

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Not building a dedicated output ring for events before workstream 010 has an actual need for one.
- **In Scope**: `event_push`/`event_drain`/`event_len`/`event_clear`'s own storage choice.

### Statement

A second ring carrying fills out to workstream 010 is explicitly deferred,
not forbidden forever — building it speculatively, before 010 names a
concrete throughput problem the plain event drain cannot handle, adds
concurrency-primitive complexity against a need nobody has confirmed yet
(see this crate's own
[`../decisions/001_no_event_drain_type.md`](../decisions/001_no_event_drain_type.md)
for the parallel reasoning against inventing a dedicated `EventDrain`
container).

### How this crate avoids it

Every function here operates on a plain `Vec<Event>` — no ring, no
bounded capacity, no producer/consumer split. Grep-verifiable:

```bash
cd module/exchange_event && command grep -n "ring_\|Ring" src/lib.rs
```

**Expected:** no output — nothing here imports or references any `ring_*`
crate.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/026_second_fill_ring_before_asked.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:925` | Pitfall in the source's Prompt 7 "Ring" list |
