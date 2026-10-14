# Responsibility: Ingress Only Through 008

### Scope

- **Purpose**: State the single path by which orders enter the book.
- **Responsibility**: Orders arrive exclusively via workstream 008's ring (many producers, one drain), never by direct call into the book or matcher.
- **In Scope**: `exchange_inbound`'s bridge to `ring_factory`/`ring_handle`.
- **Out of Scope**: The ring's own internals — 002 only consumes the drain.

**Design status**: Built — see `../inbound_path/001_inbound_path.md` and `../ring_edge/001_exchange_inbound_ring_edge.md`. `module/exchange_inbound` builds the ring (`inbound_ring`), and `exchange_core::Exchange::exchange_step` drains it; no other way to place, cancel or replace an order exists.

### Statement

002 is designed so that every order is a drained ring entry, never a direct function call from an arbitrary caller — this is what makes the matching deterministic under concurrent producers. The old direct `Exchange::submit` is gone (`../../module/exchange_core/docs/decisions/001_submit_replaced_by_ring_fed_exchange_step.md`).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1002` | Fifth bullet of Prompt 9's `responsibility` list |
