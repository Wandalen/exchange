# Exposed Item: exchange_inbound

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_inbound`.
- **Responsibility**: The single bridge from workstream 008's ring to rest-or-match.

**Design status**: Built (`module/exchange_inbound`). `InboundCmd` (`Place`/`Cancel`/`Replace`) and `inbound_flush`/`inbound_drain`/`inbound_apply`/`inbound_overflow_reject` all exist, built on `ring_factory`+`ring_handle`+`ring_types` (see `../ring_edge/001_exchange_inbound_ring_edge.md`). Three names from this list do not exist in the real build: no standalone `Inbound` type (its role is played by the re-exported `Split`/`Ends`/`Producer` instead), no `inbound_producer` function (reached through `inbound_ring` instead), and no `InboundError` type (failures are distributed across `BuildError`/`MatchError`/`RestReplaceError`/a bare `Result<(), InboundCmd>`, each already owned by the crate whose operation produced it). Full comparison: [`../../module/exchange_inbound/docs/item/readme.md`](../../module/exchange_inbound/docs/item/readme.md).

### Statement

Prompt 3 specifies the one crate permitted to depend on `ring_*`, bridging workstream 008's drain into `rest`/`match`. This now exists in the real build — see the Design status note above and [`../../module/exchange_inbound/docs/item/readme.md`](../../module/exchange_inbound/docs/item/readme.md) for exactly how closely it matches this list.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:652-661` | Crate `exchange_inbound`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
