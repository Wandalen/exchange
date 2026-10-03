# Not A 002 Crate: ring_* except as a dependency of exchange_inbound

### Scope

- **Purpose**: Name workstream 008 as a non-member of the 23-crate 002 list, with exactly one named exception.
- **Responsibility**: One of the five named exclusions from the `crate` entity.
- **In Scope**: `ring_*` crates consumed by `exchange_inbound` alone.
- **Out of Scope**: `ring_*` imported by any other 002 crate — forbidden by the boundary's "must not" list.

**Design status**: Currently vacuous — zero `ring_*` dependency exists anywhere in the family (verified via grep across all `module/*/Cargo.toml`), so there is no `exchange_inbound` crate yet to carry the one permitted exception.

### Statement

Workstream 008's ring crates are not workstream-002 crates even on the one path allowed to depend on them — `exchange_inbound` consumes `ring_core`/`ring_handle` and friends without any of them becoming part of 002's own crate list. The exception exists so this exclusion isn't read as "002 may never touch ring at all," which would contradict the inbound design entirely.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1098-1103` | Prompt 9's `not_a_002_crate` instance list, item 2 of 5 |
