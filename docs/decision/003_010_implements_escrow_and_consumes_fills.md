# Decision: 010 implements escrow and consumes fills

### Scope

- **Purpose**: Keep real balance bookkeeping out of workstream 002 entirely.
- **Responsibility**: One of the six closing decisions Prompt 9 records for workstream 002.
- **In Scope**: 002 owns only the `EscrowPort` trait/call shape; a future workstream 010 crate owns the real account/balance implementation.
- **Out of Scope**: 002 crediting a wallet directly from a match — named explicitly as a "must not."

**Design status**: Not held as drawn in the real build — see `../boundary/002_out.md`'s Design status: the real `exchange_escrow` crate implements real balance-holding itself (`Account`, `open()`, `total_cash()`, `total_asset()`) rather than deferring that to a future workstream 010 crate. This is the single most significant boundary divergence found in this comparison pass.

### Statement

The proposal's intent is a thin port in 002 with the real wallet implementation living in workstream 010, so that 002 can be exercised with "Escrow is a stub: hold succeeds, commit records the legs" in its own wall smoke without pulling in the whole economy stack. The real build instead put working balance bookkeeping directly inside 002's own `exchange_escrow` crate.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1230-1236` | Prompt 9's `decision` instance list, item 3 of 6 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:682` | The wall smoke's own "Escrow is a stub" framing |
