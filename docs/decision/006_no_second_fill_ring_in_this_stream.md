# Decision: No second fill ring in this stream

### Scope

- **Purpose**: Defer adding a second ring (for publishing fills outward) until a real consumer asks for it.
- **Responsibility**: One of the six closing decisions Prompt 9 records for workstream 002.
- **In Scope**: `exchange_event`'s drain stays a plain in-memory drain for now.
- **Out of Scope**: Building a second ring speculatively inside 002 — named explicitly as both a "not_a_002_crate" item and a pitfall.

**Design status**: Held by omission — no second ring exists anywhere in the real crates, consistent with this decision; `exchange_event` is folded into `exchange_types`/`exchange_core::events()` as a plain drain.

### Statement

002 deliberately stops at one ring (inbound only) and explicitly defers a second ring for outbound fills until workstream 010 or another consumer actually needs it — named three times in the source (boundary, pitfall tape, and here) as a YAGNI-shaped guard against building speculative infrastructure for a consumer that does not yet exist.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1230-1236` | Prompt 9's `decision` instance list, item 6 of 6 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:925` | The matching pitfall-tape entry |
