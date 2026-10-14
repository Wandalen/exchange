# Decision: FOK is all or nothing

### Scope

- **Purpose**: Forbid a Fill-Or-Kill order from ever partially filling before being rejected.
- **Responsibility**: One of the six closing decisions Prompt 9 records for workstream 002.
- **In Scope**: `match_fok_check`'s admission test, run before any fill for that order is emitted.
- **Out of Scope**: Any other TIF value's partial-fill behavior — only FOK carries this all-or-nothing guarantee.

**Design status**: Not yet applicable — TIF, and therefore FOK, is not implemented (see `../tif/003_fok.md`); there is no `match_fok_check` to verify this decision against yet.

### Statement

A Fill-Or-Kill order must either fill its entire quantity in one pass or leave the book completely unchanged — never partially fill and then reject. The source's own pitfall tape names this exact ordering bug ("FOK that fills part, then rejects") as the one this decision exists to forbid.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1230-1236` | Prompt 9's `decision` instance list, item 5 of 6 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:913` | The matching pitfall-tape entry |
