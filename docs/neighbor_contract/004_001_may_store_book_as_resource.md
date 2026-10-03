# Neighbor Contract: 001 May Store a Book as a Resource

### Scope

- **Purpose**: State the optional, deferred relationship between 002's book and workstream 001's ECS.
- **Responsibility**: 001 (Demiurg) may later hold a `Book` as an ECS resource; 002 does not depend on 001 to function.
- **In Scope**: A possible future hosting arrangement.
- **Out of Scope**: Any current dependency — 002 does not require ECS columns.

### Statement

This is a one-directional, optional, future relationship: workstream 001 may choose to store a `Book` as a resource inside its ECS world later, but 002's own design never assumes this — the book is closed, plain data specifically so that it *can* be dropped into a VM resource slot without 002 knowing or caring that it happened.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1091` | Fourth bullet of Prompt 9's `neighbor_contract` list |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:146` | "001 Demiurg. Книга може пізніше лежати в resource, але матч не потребує колонок" (the book may later live in a resource, but the match needs no columns) |
