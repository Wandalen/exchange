# Not A 002 Crate: a fill ring

### Scope

- **Purpose**: Name a second (outbound) ring, for publishing fills, as something 002 does not build in this pass.
- **Responsibility**: One of the five named exclusions from the `crate` entity.
- **In Scope**: Clarifying `exchange_event` stays a plain drain, not a ring-backed publisher.
- **Out of Scope**: A future fill ring, if workstream 010 or another consumer eventually asks for one — this exclusion is scoped to "not now," not "never."

**Design status**: Held — no second ring exists anywhere in the real crates; see `../decision/006_no_second_fill_ring_in_this_stream.md`.

### Statement

A second ring carrying fills outward to 010 is explicitly deferred rather than built speculatively alongside the one inbound ring — the same YAGNI reasoning the `decision` entity's closing ruling states directly: no second fill ring in this stream, full stop, until a concrete consumer needs it.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1098-1103` | Prompt 9's `not_a_002_crate` instance list, item 5 of 5 |
