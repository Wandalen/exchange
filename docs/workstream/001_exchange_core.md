# Workstream: 002 Exchange Core

### Scope

- **Purpose**: Name and scope the workstream that owns the price-time order book and the matching engine.
- **Responsibility**: Workstream 002 — Exchange Core, the matching engine. Rest, cancel, replace, match, partial fills, escrow port, fills and rejects.
- **In Scope**: The book, the matcher, escrow port, fill/reject/cancel-ack events, ingress from workstream 008's ring.
- **Out of Scope**: Wallets and balances (010), decimal arithmetic implementation (006), the ECS virtual machine (001).

### Statement

Workstream 002 is Exchange Core: the matching engine, identified in the source transcript when the user asked "do we have such [a matching-engine] workstream or not?" and Grok confirmed it as 002. It owns the price-time book — rest, cancel, replace, match, partial fills, escrow port, fills and rejects — but does not own wallets (workstream 010) or decimal math (workstream 006). Its compile dependencies are exactly two: workstream 006 for prices and quantities, and workstream 008 for ring-based ingress only (the book and matcher themselves never import `ring_*`). Prompts 1 and 2 (hard problems/features/boundaries, and the crate list) were drafted earlier in a prior session, before 006 was closed; this transcript resumes at prompt 3 once 006 closed and carries the workstream through to prompt 9, closing 002 before opening workstream 010's prompt 1.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:118-133` | The user asking whether the matching-engine workstream exists; Grok confirms it is 002 and that prompts 1-2 were already drafted, resuming at prompt 3 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:136-149` | 002's two compile dependencies: 006 (exact arithmetic) and 008 (ring, inbound only) |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:994-995` | Prompt 9's `workstream` doc instance: "002 Exchange Core" |
