# Escrow Port: hold_release

### Scope

- **Purpose**: Return a lock to available balance without it ever becoming a transfer.
- **Responsibility**: One of the three `EscrowPort` trait methods.
- **In Scope**: Cancel, IOC leftover, and FOK reject — every path that ends a hold without a fill.
- **Out of Scope**: Converting a hold into a settled transfer — that is `hold_commit`'s job.

**Design status**: Named differently in the real build — `exchange_escrow::Escrow` (`module/exchange_escrow/src/lib.rs`) exposes `release()`, matching this operation's intent closely by name as well as behavior.

### Statement

`hold_release` reverses a reservation that never became a fill — the funds or asset it locked return to available balance exactly as if the hold had never happened. The proposal's pitfall tape names forgetting this release on cancel, IOC leftover, or FOK reject as a distinct bug from forgetting it after a fill, because a fill should call `hold_commit` instead, never `hold_release`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1172-1175` | Prompt 9's `escrow_port` instance list, item 2 of 3 |
