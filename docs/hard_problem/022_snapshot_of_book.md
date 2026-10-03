# Hard Problem: Snapshot Of Book

### Scope

- **Purpose**: Resting orders are copyable rows.
- **Responsibility**: Represent resting orders as copyable rows so the book can be saved, replayed, and extracted.

### Statement

Save, replay, and extraction all need resting orders as copyable rows; without a snapshot, the market dies the moment the process restarts.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:292-295` | Hard problem 22 in the source's Prompt 1 answer for workstream 002 |
