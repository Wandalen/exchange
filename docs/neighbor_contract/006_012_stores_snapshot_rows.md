# Neighbor Contract: 012 Stores Snapshot Rows

### Scope

- **Purpose**: State that persistence of book state is workstream 012's job, not a second copy of the book.
- **Responsibility**: 012 stores the plain snapshot rows 002 produces; it never becomes a second live book.
- **In Scope**: `exchange_snap`'s `BookSnap`/`RestRow` as the handoff format.
- **Out of Scope**: Any live matching logic living in 012.

**Design status**: Not built — no `exchange_snap` crate or snapshot type exists in the real family yet.

### Statement

012 (the save/replay format workstream, referenced but not detailed in this transcript) is the consumer of 002's snapshot rows — plain, copyable records of resting orders and the instrument's tick — never a second order book with its own matching logic. This keeps persistence a one-way, read-only export from 002's perspective.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1093` | Sixth bullet of Prompt 9's `neighbor_contract` list |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:470-474` | `exchange_snap`'s crate definition: "не формат файлу 012" (not 012's file format itself, just rows for it to store) |
