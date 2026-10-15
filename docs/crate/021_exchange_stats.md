# Crate: exchange_stats

### Scope

- **Purpose**: Rests, fills from this call, rejects.
- **Responsibility**: Give the hot path a counter it can read.
- **In Scope**: The counters themselves.
- **Out of Scope**: Any policy — this crate decides nothing.

**Design status**: Built, in its own `exchange_stats` crate — `BookStats` matches the proposal's `{ rests, fills, rejects, cancels }` shape exactly, with `stats_rest_add`/`stats_cancel_add` added for symmetry (the proposal's own function list left both uncovered). Verified built-vs-proposed comparison: [`../../module/exchange_stats/docs/item/readme.md`](../../module/exchange_stats/docs/item/readme.md).

### Statement

Without this crate, the hot path runs blind — nobody can see what it is actually doing. It closes hard problem 14 (hot path) and feature 23 (`BookStats`). It depends on `exchange_id`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:476-481` | Crate 21 in the source's Prompt 2 answer for workstream 002 |
