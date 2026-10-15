# Decision: Compile deps are 006 and 008 only

### Scope

- **Purpose**: Fix workstream 002's Cargo-level dependency surface to exactly two neighbors.
- **Responsibility**: One of the six closing decisions Prompt 9 records for workstream 002.
- **In Scope**: `exact_arith` (006) and, on `exchange_inbound` alone, a named subset of `ring_*` (008).
- **Out of Scope**: Workstream 001 (Demiurg), 004 (time), 007 (sockets), 010 (wallets) — all explicitly named as non-dependencies.

**Design status**: Half held — `exchange_core`'s real `Cargo.toml` depends on `exact_arith` directly, matching 006. The 008 half does not yet apply: zero `ring_*` dependency exists anywhere in the family (verified via grep across all `module/*/Cargo.toml`), consistent with `exchange_inbound` not existing yet.

### Statement

002 compiles against exactly two neighboring workstreams: 006 for every price and quantity type, and 008 for inbound-only ring access, with explicit non-dependencies called out for 001, 004, 007, and 010 — each of those relationships exists at the data/consumer level, never as a Cargo path dependency.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1230-1236` | Prompt 9's `decision` instance list, item 1 of 6 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:138-149` | The earlier conversational answer this decision formalizes |
