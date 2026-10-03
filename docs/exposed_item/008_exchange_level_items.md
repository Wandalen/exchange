# Exposed Item: exchange_level

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_level`.
- **Responsibility**: One price, FIFO rest — the queue a book keeps at a single price point.

**Design status**: Built, in its own `exchange_level` crate — this summary predates that extraction. All seven functions (`level_new`/`level_push`/`level_pop_front`/`level_remove`/`level_len`/`level_qty_sum`/`level_empty_is`) exist; only `LevelError` is declined. Verified built-vs-proposed comparison, checked directly against the primary source rather than this summary: [`../../module/exchange_level/docs/item/readme.md`](../../module/exchange_level/docs/item/readme.md).

### Statement

Prompt 3 specifies a `Level`/`LevelNode` pair implementing one price's FIFO queue, with seven functions and a `LevelError`. See the per-crate doc linked above for the current, verified comparison.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:572-577` | Crate `exchange_level`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
