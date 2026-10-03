# Hard Problem: Full Order Of Inbound

### Scope

- **Purpose**: The ring's drain has a stable order before the match runs.
- **Responsibility**: Give the ring's drain a stable total order before the match ever runs on it.

### Statement

Two producers draining into the match without a stable total order would each produce a different match — 008 existing isn't enough on its own; the drain itself needs total order before matching runs.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:297-300` | Hard problem 23 in the source's Prompt 1 answer for workstream 002 |
