# Exposed Item: exchange_level

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_level`.
- **Responsibility**: One price, FIFO rest — the queue a book keeps at a single price point.

**Design status**: Built — all seven `level_*` functions; `LevelError` declined, nodes in a `VecDeque` rather than an intrusive list. Built-vs-proposed comparison: [`../../module/exchange_level/docs/item/readme.md`](../../module/exchange_level/docs/item/readme.md).

### Statement

Prompt 3 specifies a `Level`/`LevelNode` pair implementing one price's FIFO queue, with seven functions and a `LevelError`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:572-577` | Crate `exchange_level`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
