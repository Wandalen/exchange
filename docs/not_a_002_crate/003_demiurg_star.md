# Not A 002 Crate: demiurg_*

### Scope

- **Purpose**: Name workstream 001 (the ECS virtual machine) as a non-member of the 23-crate 002 list, and as a non-dependency at all today.
- **Responsibility**: One of the five named exclusions from the `crate` entity.
- **In Scope**: Clarifying 002 compiles independently of Demiurg.
- **Out of Scope**: A future possibility — `neighbor_contract` separately notes "001 may store a book as a resource later," which is not a dependency today.

**Design status**: Held — no `demiurg_*` dependency exists anywhere in the real crate family.

### Statement

Unlike `exact_*` and `ring_*`, `demiurg_*` is not merely excluded from the crate list — it is not a compile dependency of 002 at all, today or in the proposal. The match engine does not need ECS columns to function, and the book may only later be *stored inside* a Demiurg resource without that changing what 002 itself builds.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1098-1103` | Prompt 9's `not_a_002_crate` instance list, item 3 of 5 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:144-146` | The earlier conversational answer naming 001 as a non-dependency |
