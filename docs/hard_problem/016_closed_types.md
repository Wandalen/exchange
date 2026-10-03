# Hard Problem: Closed Types

### Scope

- **Purpose**: Ids and the book are plain data, without host pointers.
- **Responsibility**: Keep ids and the book as plain data with no host pointers, so the book can live inside the VM and be snapshotted.

### Statement

Ids and the book must be plain data with no host pointers, so the book can be placed inside the VM and snapshotted. Without closed types, this workstream can't live inside workstream 001's storage.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:262-265` | Hard problem 16 in the source's Prompt 1 answer for workstream 002 |
