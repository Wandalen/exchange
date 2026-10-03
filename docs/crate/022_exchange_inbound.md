# Crate: exchange_inbound

### Scope

- **Purpose**: The single bridge to workstream 008 — TLS flush, drain in total order, overflow becomes a reject, then rest or match.
- **Responsibility**: Own the one path orders take from the ring into the book.
- **In Scope**: The bridge itself. Does not own `ring_claim` and does not implement gating.
- **Out of Scope**: Owning `ring_claim` or gating.

**Design status**: Not built — zero `ring_*` dependency exists anywhere in the family (verified via grep across every `module/*/Cargo.toml`); no bridge to workstream 008 exists at all.

### Statement

Without this crate, match ends up getting called mid-system, or the book itself has to import the ring directly. It closes hard problems 11 (inbound at aeon edge), 23 (total order of inbound), and 24 (ring overflow is a reject), and features 29 (inbound flush, drain, then rest or match) and 30 (overflow to reject). Its ring edge is exactly `ring_core` or `ring_handle`, `ring_tls`, `ring_flush`, `ring_batch`, `ring_overflow`, `ring_poll` — never `ring_spsc` as the market path, never `ring_bench`. It depends on `exchange_order`, `exchange_rest`, `exchange_match`, `exchange_event`, plus those named `ring_*` crates.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:482-487` | Crate 22 in the source's Prompt 2 answer for workstream 002 |
