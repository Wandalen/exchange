# Smoke Demo: smoke_exchange_book

### Scope

- **Purpose**: Grade phase P30, the wall — the one proposed demo that closes the workstream.
- **Responsibility**: Exercise most hard problems and features together behind one golden print.

**Design status**: The real build's smoke lane is named `smoke_exchange_core`, not `smoke_exchange_book`, and covers one order across five crates with a no-cross control arm — much thinner than this proposed wall's full scenario (FIFO, partials, IOC/FOK, duplicate id, halt/resume, self-trade, two-producer ring-drain determinism, ring overflow, conservation, depth_top).

### Statement

`smoke_exchange_book` is the merge gate: headless, one file, escrow stubbed, ingress through `exchange_inbound` and a two-producer ring. It covers price-time priority, partial fills, all three TIF dispositions, duplicate rejection, halt/resume, self-trade prevention, deterministic concurrent drain, ring overflow, conservation, and depth — the single widest-coverage check in the whole workstream.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:879` | Phase P30's smoke, naming the wall, in the source's Prompt 6 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:680-772` | The wall smoke's full scene and golden print, from Prompt 4 |
