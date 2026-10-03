# Decision: Ring edge lives only on exchange_inbound

### Scope

- **Purpose**: Confine every `ring_*` dependency to a single crate, so the book and matcher never see it.
- **Responsibility**: One of the six closing decisions Prompt 9 records for workstream 002.
- **In Scope**: `exchange_inbound` alone carries the `ring_edge` dependency set.
- **Out of Scope**: `exchange_book` and `exchange_match` importing `ring_*` directly — named as a "must not" boundary item and a pitfall both.

**Design status**: Vacuously held for now — zero `ring_*` dependency exists anywhere in the family (verified via grep across all `module/*/Cargo.toml`), so the prohibition on `exchange_book`/`exchange_match` importing ring is trivially satisfied by nothing importing it at all yet.

### Statement

The ring dependency is deliberately isolated to one crate, `exchange_inbound`, rather than let the book or matcher reach for it directly — reinforced three times in the source: as hard problem/boundary, as a pitfall ("`ring_*` imported by `exchange_book` or `exchange_match`"), and here as a standalone decision.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1230-1236` | Prompt 9's `decision` instance list, item 2 of 6 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:918-919` | The matching pitfall-tape entry |
