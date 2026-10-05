# Smoke Demo: smoke_exchange_book

### Scope

- **Purpose**: Grade phase P30, the wall — the one proposed demo that closes the workstream.
- **Responsibility**: Exercise most hard problems and features together behind one golden print.

**Design status**: Built, as `module/smoke_exchange_book`, replacing the thinner `smoke_exchange_core` lane (Stage 10 of this family's refactor — see git history). Matches the proposed scenario's coverage: multi-level price-time priority, IOC/FOK disposition, duplicate-id rejection, halt/resume, self-trade prevention, two-producer ring-drain determinism, ring overflow, conservation, and `depth_get`. One divergence from the proposal's own text: escrow is real, not stubbed — this family's Stage 5 decision keeps `exchange_escrow` holding genuine balances throughout, so the wall's accounts are funded and settled for real rather than through a stand-in.

### Statement

`smoke_exchange_book` is the merge gate: headless, one file, escrow stubbed, ingress through `exchange_inbound` and a two-producer ring. It covers price-time priority, partial fills, all three TIF dispositions, duplicate rejection, halt/resume, self-trade prevention, deterministic concurrent drain, ring overflow, conservation, and depth — the single widest-coverage check in the whole workstream.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:879` | Phase P30's smoke, naming the wall, in the source's Prompt 6 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:680-772` | The wall smoke's full scene and golden print, from Prompt 4 |
