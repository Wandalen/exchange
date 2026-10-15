# Crate: exchange_core

### Scope

- **Purpose**: The facade.
- **Responsibility**: Give the caller one crate to depend on instead of twenty.
- **In Scope**: Composition only. Does not import `ring_*` — that edge stays on `exchange_inbound`.
- **Out of Scope**: Owning any hard problem of its own.

**Design status**: Built as the real `exchange_core` crate — the facade, depending on `exchange_types`, `exchange_book`, `exchange_fill`, `exchange_match`, `exchange_order`, `exchange_side`, `exchange_escrow`, `exchange_id`, `exchange_seq`, `exchange_tif`, and `exact_arith` directly (`module/exchange_core/Cargo.toml`; `exchange_fill`/`exchange_order`/`exchange_side` added when `exchange_types`' own re-export aggregator role retired, since the facade's re-exports of `Trade`/`Event`/etc. now source from the real leaf crates directly instead).

### Statement

Without a facade, the caller has to assemble twenty crates by hand. This crate owns no hard problem of its own — its feature is composition itself. It depends on `exchange_spec`, `exchange_book`, `exchange_rest`, `exchange_match`, `exchange_depth`, `exchange_halt`, `exchange_event`, `exchange_snap`, `exchange_stats`, and `exchange_inbound`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:488-493` | Crate 23 in the source's Prompt 2 answer for workstream 002 |
