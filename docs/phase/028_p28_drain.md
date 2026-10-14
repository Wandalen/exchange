# Phase: P28 — Deterministic two-producer drain

### Scope

- **Purpose**: Prove inbound drain order is deterministic across runs, not thread-completion-order-dependent.
- **Responsibility**: Two producers draining in full order give the same checksum on repeated runs.

### Statement

The twenty-eighth phase's one new contract: two producers publishing concurrently still drain in one stable, total order — run it twice and the resulting checksum is identical both times.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:809` | Phase P28 in the source's Prompt 5 answer for workstream 002 |
