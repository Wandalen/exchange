# Exposed Item: exchange_inbound

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_inbound`.
- **Responsibility**: The single bridge from workstream 008's ring to rest-or-match.

**Design status**: Not built — no real crate exists for this; nothing to compare. No `InboundCmd`, `Inbound`, `inbound_producer`/`inbound_flush`/`inbound_drain`/`inbound_apply`/`inbound_overflow_reject`, or `InboundError` exists anywhere. Confirmed via grep across every `module/*/Cargo.toml` in the family: zero `ring_*` dependency exists at all. `exchange_core::Exchange::submit` is called directly by a caller today — there is no ring-fed ingress path, no TLS-flush/drain step, and no overflow-as-reject handling (see `../ring_edge/001_exchange_inbound_ring_edge.md` and `../inbound_path/001_inbound_path.md`).

### Statement

Prompt 3 specifies the one crate permitted to depend on `ring_*`, bridging workstream 008's drain into `rest`/`match`. None of this exists in the real build — there is no ring integration of any kind, and submission is a direct synchronous function call.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:652-661` | Crate `exchange_inbound`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
