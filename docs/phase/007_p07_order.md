# Phase: P07 — Order with sequence

### Scope

- **Purpose**: Prove an order carries a sequence number and rejects an empty quantity.
- **Responsibility**: Submission fails on zero quantity; arrival claims a sequence.

**Design status**: Proven by `demo_p07_order` (`module/smoke_exchange_phases/src/bin/demo_p07_order.rs`), but not exactly as this phase was first worded: `exchange_order::Order` has no `Sequence`/arrival field and no validating constructor — per `docs/crate/006_exchange_order.md`'s own documented divergence, quantity validation lives at `exchange_core::submit`'s boundary, and arrival sequencing is assigned separately (by whatever rests the order — `exchange_level::LevelNode`'s own `arrival` field, for example) rather than carried on `Order` itself. The demo proves the same two facts the phase cares about — a fresh order's arrival claims sequence `1`, and a zero-quantity submission is refused — through the real call sites that actually do each, not through `Order`'s own construction.

### Statement

The seventh phase's one new contract: an order's arrival claims a sequence number, and a zero-quantity order is rejected rather than accepted as a no-op.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:788` | Phase P07 in the source's Prompt 5 answer for workstream 002 |
