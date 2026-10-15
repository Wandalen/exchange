# Exposed Item: exchange_stats

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_stats`.
- **Responsibility**: Counting rests, fills, and rejects for hot-path visibility.

**Design status**: Built, in its own `exchange_stats` crate — this summary predates that extraction. `BookStats`/`stats_zero`/`stats_fill_add`/`stats_reject_add`/`stats_snapshot` all exist, matching the proposal's named surface exactly; `stats_rest_add`/`stats_cancel_add` are added beyond it, for symmetry (the proposal's own function list left both `rests` and `cancels` with no incrementing function). Verified built-vs-proposed comparison: [`../../module/exchange_stats/docs/item/readme.md`](../../module/exchange_stats/docs/item/readme.md).

### Statement

Prompt 3 specifies a dedicated running-counter crate for hot-path observability. See the per-crate doc linked above — the real build has exactly this, in its own crate, plus two symmetry-driven additions.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:647-650` | Crate `exchange_stats`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
