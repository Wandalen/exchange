# Responsibility: Ingress Only Through 008

### Scope

- **Purpose**: State the single path by which orders enter the book.
- **Responsibility**: Orders arrive exclusively via workstream 008's ring (many producers, one drain), never by direct call into the book or matcher.
- **In Scope**: `exchange_inbound`'s bridge to `ring_core`/`ring_handle`.
- **Out of Scope**: The ring's own internals — 002 only consumes the drain.

**Design status**: Not built — see `../inbound_path/001_inbound_path.md`. No `exchange_inbound` crate and no `ring_*` dependency exist anywhere in the real family.

### Statement

002 is designed so that every order is a drained ring entry, never a direct function call from an arbitrary caller — this is what makes the matching deterministic under concurrent producers. The real build has no ingress mechanism at all yet; callers invoke `Exchange::submit` directly.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1002` | Fifth bullet of Prompt 9's `responsibility` list |
