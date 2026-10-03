# Inbound Path: TLS Flush → Drain → Rest Or Match → Overflow As Reject

### Scope

- **Purpose**: Define the one path by which an order reaches the book, so no caller can bypass it.
- **Responsibility**: A fixed four-step sequence `exchange_inbound` is solely responsible for.
- **In Scope**: Thread-local staging flush, total-order ring drain, dispatch to rest or match, and overflow handling.
- **Out of Scope**: Claiming ring slots or implementing gating — `exchange_inbound` consumes the ring, it does not own it.

**Design status**: Not implemented — zero `ring_*` dependency exists anywhere in `substrate/exchange` (verified via grep across all `module/*/Cargo.toml`). No `exchange_inbound` crate exists.

### Statement

The path is four steps, always in this order: (1) a thread-local staging buffer flushes into the ring, (2) the ring is drained in total order — stable regardless of which producer thread happened to publish first, (3) each drained command is then either rested or matched, and (4) if the ring itself is full, the producer gets a `Reject`, never a silent drop. This path is the only legal way an order enters the book — the matcher never claims ring slots itself, and nothing inside it may block or park.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1188-1192` | Prompt 9's `inbound_path` instance, four steps |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:482-486` | `exchange_inbound`'s crate definition in Prompt 2, same four-step shape |
