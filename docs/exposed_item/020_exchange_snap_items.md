# Exposed Item: exchange_snap

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_snap`.
- **Responsibility**: Plain rows of resting orders, with the tick supplied by the caller.

**Design status**: Built — `RestRow`, `BookSnap`, `snap_take`, `snap_len`;
`RestRow` adds `side`, `tick` is a `u64`, and there is no `SnapError`.
Built-vs-proposed comparison:
[`../../module/exchange_snap/docs/item/readme.md`](../../module/exchange_snap/docs/item/readme.md).

### Statement

Prompt 3 specifies `RestRow { order, price, qty }`, `BookSnap { instrument,
tick, rows }`, `snap_take`, `snap_len` and `SnapError { Full }`, for
save/replay/extract use.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:641-645` | Crate `exchange_snap`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
