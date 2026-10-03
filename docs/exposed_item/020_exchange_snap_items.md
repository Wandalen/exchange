# Exposed Item: exchange_snap

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_snap`.
- **Responsibility**: Plain rows of resting orders, with the tick supplied by the caller.

**Design status**: Built, in its own `exchange_snap` crate — this summary predates that extraction. `RestRow`/`BookSnap`/`snap_take`/`snap_len` all exist, matching the proposal's fields exactly; the two real divergences are a dropped `exchange_order` dependency and no `SnapError`. Verified built-vs-proposed comparison: [`../../module/exchange_snap/docs/item/readme.md`](../../module/exchange_snap/docs/item/readme.md).

### Statement

Prompt 3 specifies a snapshot-row pair for save/replay/extract use. See the per-crate doc linked above — the real build has exactly this, in its own crate.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:641-645` | Crate `exchange_snap`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
