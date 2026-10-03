# Exposed Item: exchange_depth

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_depth`.
- **Responsibility**: Top-N book depth without a full walk.

**Design status**: Built, in its own `exchange_depth` crate — this summary predates that extraction. `LevelView`, `Depth`, `depth_top`, and `DepthError { BadN }` all exist, matching the proposal exactly. Verified built-vs-proposed comparison: [`../../module/exchange_depth/docs/item/readme.md`](../../module/exchange_depth/docs/item/readme.md).

### Statement

Prompt 3 specifies a dedicated `Depth`/`depth_top` pair avoiding a full book walk. See the per-crate doc linked above — the real build has exactly this, in its own crate.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:625-629` | Crate `exchange_depth`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
