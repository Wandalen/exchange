# Pitfall: ring_* imported by exchange_book or exchange_match

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Letting the book or matching crate depend on any `ring_*` crate directly.
- **In Scope**: Ring

**Design status**: Trivially avoided — zero `ring_*` dependency exists anywhere in the family (verified via grep across every `module/*/Cargo.toml`). No crate in the real build imports `ring_*` at all, including on the inbound path this pitfall's boundary would otherwise permit.

### Statement

The ring is meant to touch only the inbound edge (`exchange_inbound` in the proposal), never the book or the matcher directly — if either of those crates imported `ring_*`, the core matching logic would carry a concurrency-primitive dependency it has no reason to need, and the layering the whole design relies on would be gone.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:919` | Pitfall in the source's Prompt 7 "Ring" list |
