# Pitfall: Building the wallet inside this stream

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Implementing real account balances as part of workstream 002 instead of leaving that to workstream 010.
- **In Scope**: Process

**Design status**: Worth flagging directly — the real `exchange_escrow` crate (`module/exchange_escrow/src/lib.rs`) does hold real `Account` balances (`open()`, `total_cash()`, `total_asset()`) rather than deferring that to a future workstream 010 crate. See `../crate/012_exchange_escrow.md` and `../boundary/002_out.md` for the full account of this divergence — whether it counts as "building the wallet inside this stream" in the sense this pitfall warns against is a scope question the source material doesn't resolve either way.

### Statement

Workstream 002's own boundary explicitly excludes balances — they are workstream 010's job — so building wallet logic here duplicates work that belongs elsewhere and risks diverging from whatever account model 010 eventually settles on.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:940` | Pitfall in the source's Prompt 7 "Process" list |
