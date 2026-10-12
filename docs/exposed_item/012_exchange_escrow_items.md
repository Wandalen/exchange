# Exposed Item: exchange_escrow

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_escrow`.
- **Responsibility**: The hold/release/commit port between the book and a wallet.

**Design status**: Built as `Escrow`, a concrete ledger holding real
balances, with `reserve`/`release`/`settle` for `hold_try`/`hold_release`/
`hold_commit`, a 10-variant `EscrowError`, and `Holding`/`Account`/`Conserved`.
The port is `exchange_core::EscrowPort`, which `Escrow` implements; no `Hold`
type. Built-vs-proposed comparison:
[`../../module/exchange_escrow/docs/item/readme.md`](../../module/exchange_escrow/docs/item/readme.md).

### Statement

Prompt 3 specifies a minimal three-method port a future crate implements, with a three-variant error.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:597-600` | Crate `exchange_escrow`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
