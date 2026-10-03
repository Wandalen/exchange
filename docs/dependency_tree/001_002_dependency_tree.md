# Dependency Tree: Workstream 002's 23-Crate Graph

### Scope

- **Purpose**: Record the complete proposed dependency graph in one place, as distributed knowledge otherwise scattered across 23 separate `crate/` entries.
- **Responsibility**: Every one of the 23 crates' direct dependencies, plus the roots/trunk/bridge/facade summary.
- **In Scope**: Compile-time `path` dependencies between the 23 proposed crates.
- **Out of Scope**: The real build's actual dependency graph — see `../crate/` for the as-built comparison per crate.

### Statement

Six crates are roots with no dependencies at all: `exchange_id`, `exchange_side`, `exchange_tif`, `exchange_stp`, `exchange_seq`, `exchange_cap`. From there the graph runs through a trunk (`spec → order → level → book → rest / match`), a bridge from `exchange_inbound` out to six named `ring_*` crates, and converges on the `exchange_core` facade.

```text
exchange_id
exchange_side
exchange_tif
exchange_stp
exchange_seq
exchange_cap
exchange_spec          → exchange_id          (+ 006 types)
exchange_order         → exchange_id, exchange_side, exchange_tif
exchange_idem          → exchange_id
exchange_level         → exchange_order, exchange_seq
exchange_book          → exchange_level, exchange_spec, exchange_id
exchange_escrow        → exchange_id, exchange_order
exchange_fill          → exchange_id, exchange_order
exchange_conserve      → exchange_fill        (+ 006 types)
exchange_rest          → book, idem, cap, escrow, spec
exchange_match         → book, escrow, fill, stp, tif, conserve
exchange_depth         → exchange_book
exchange_halt          → exchange_spec, exchange_book
exchange_event         → exchange_fill
exchange_snap          → exchange_order, exchange_book
exchange_stats         → exchange_id
exchange_inbound       → order, rest, match, event
                         - ring_core|ring_handle, ring_tls, ring_flush,
                           ring_batch, ring_overflow, ring_poll
exchange_core          → spec, book, rest, match, depth, halt,
                         event, snap, stats, inbound
```

Roots: `exchange_id`, `exchange_side`, `exchange_tif`, `exchange_stp`, `exchange_seq`, `exchange_cap`.
Trunk: `spec → order → level → book → rest / match`.
Bridge: `exchange_inbound → ring`.
Facade: `exchange_core`.

**Design status**: The real build's graph is far shallower — `exchange_core` depends directly on `exchange_types`, `exchange_book`, `exchange_match`, `exchange_escrow`, and `exact_arith`, with no intermediate layers (no `spec`/`level`/`idem`/`rest` as separate crates) and no `exchange_inbound`/ring bridge at all.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:494-523` | The full ASCII dependency tree |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1130-1134` | Prompt 9's `dependency_tree` instance: the Roots/Trunk/Bridge/Facade summary |
