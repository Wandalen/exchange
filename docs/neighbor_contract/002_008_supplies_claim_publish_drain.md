# Neighbor Contract: 008 Supplies Claim/Publish/Drain

### Scope

- **Purpose**: State exactly what workstream 008 (ring) provides to 002, and what it withholds.
- **Responsibility**: Claim, publish, and drain mechanics for the inbound ring; gating itself stays in 008.
- **In Scope**: `exchange_inbound`'s consumption of `ring_core`/`ring_handle`, `ring_tls`, `ring_flush`, `ring_batch`, `ring_overflow`, `ring_poll`.
- **Out of Scope**: 002 never implements its own gating (the "producer does not outrun the slowest consumer" logic) — that is `ring_gating`'s job inside 008.

**Design status**: Not yet consumed — see `../inbound_path/001_inbound_path.md`. No `ring_*` dependency exists anywhere in the real family yet.

### Statement

008 hands 002 a working claim/publish/drain mechanism; 002's only obligation is to drain it in total order and turn overflow into a `Reject`, never a silent drop. 002 does not own or reimplement gating — the ring's own backpressure between producer and consumer is entirely 008's concern.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1089` | Second bullet of Prompt 9's `neighbor_contract` list |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:80` | Earlier in the transcript: "For exchange the ring only touches exchange_inbound... The book and matcher do not import ring_*" |
