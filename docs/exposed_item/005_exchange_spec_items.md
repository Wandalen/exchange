# Exposed Item: exchange_spec

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_spec`.
- **Responsibility**: Instrument identity, tick/lot/halt metadata, and price/quantity snapping.

**Design status**: Built as listed, except `SpecError`'s third variant
(`Overflow`, not `Snap` — it aliases `exact_arith::SnapError`), plus
`price_fits`/`qty_fits`. Full writeup moved to
[`../../module/exchange_spec/docs/item/readme.md`](../../module/exchange_spec/docs/item/readme.md).

### Statement

Prompt 3 specifies `AssetId(u32)`, `InstrumentSpec { id, base, quote, tick, lot, halted }`, construction/halt-query functions, `price_snap`/`qty_snap`, and a `SpecError`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:554-559` | Crate `exchange_spec`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
