# Not A 002 Crate: wallet / fee sink

### Scope

- **Purpose**: Name the wallet/fee-sink implementation as workstream 010's territory, never something 002 builds.
- **Responsibility**: One of the five named exclusions from the `crate` entity.
- **In Scope**: Clarifying that 002 emits `Fill` events only; settling them into a wallet is out of scope.
- **Out of Scope**: Anything about how 010 implements the wallet — that is its own future design.

**Design status**: Not held as cleanly as intended — see `../boundary/002_out.md` and `../decision/003_010_implements_escrow_and_consumes_fills.md`: the real `exchange_escrow` crate already holds real account balances inside 002, which sits closer to "wallet" territory than this exclusion anticipates.

### Statement

Workstream 002's own pitfall tape repeats this exclusion as a rule ("Building the wallet inside this stream") and as a process-level warning against a wallet sneaking in before the merge gate is reached. The real build's `exchange_escrow` crate is the one place this line has already blurred — see the Design status above.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1098-1103` | Prompt 9's `not_a_002_crate` instance list, item 4 of 5 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:940` | The matching pitfall-tape entry, "Building the wallet inside this stream" |
