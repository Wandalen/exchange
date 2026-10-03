# Crate: exchange_match

### Scope

- **Purpose**: A taker against the opposite ladder, partials, TIF, STP.
- **Responsibility**: Own the crossing loop itself.
- **In Scope**: Crossing. No wallet credit, no `ring_*` import.
- **Out of Scope**: Crediting a wallet.

**Design status**: Built as the real `exchange_match` crate — `Crossing`, `cross()` (`module/exchange_match/src/lib.rs`).

### Statement

Without this crate, there is no engine at all. It closes hard problems 2 (price-time), 3 (partial fills), 6 (deterministic match), 10 (self-trade), and 20 (time in force), and features 9 (`match_in`), 10 (partial fill, rest the leftover), 15 (STP policy), and 16 (TIF). It depends on `exchange_book`, `exchange_escrow`, `exchange_fill`, `exchange_stp`, `exchange_tif`, and `exchange_conserve`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:446-451` | Crate 16 in the source's Prompt 2 answer for workstream 002 |
