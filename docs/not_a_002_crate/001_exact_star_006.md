# Not A 002 Crate: exact_* (006)

### Scope

- **Purpose**: Name workstream 006 itself as a non-member of the 23-crate 002 list, even though 002 depends on it.
- **Responsibility**: One of the five named exclusions from the `crate` entity.
- **In Scope**: Clarifying that `exact_*` crates are a dependency, not a crate this workstream owns or ships.
- **Out of Scope**: Any statement about what `exact_*` itself contains — that is workstream 006's own territory.

**Design status**: Held — `exchange_core`'s real `Cargo.toml` depends on `exact_arith` as a git dependency pinned in the root `Cargo.toml`; no `exact_*` crate lives inside `substrate/exchange/module/`.

### Statement

Depending on workstream 006 for every price and quantity type does not make `exact_*` a workstream-002 crate — it remains entirely 006's own family, consumed rather than owned. This distinction matters because the `crate` entity enumerates only what 002 itself builds.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1098-1103` | Prompt 9's `not_a_002_crate` instance list, item 1 of 5 |
