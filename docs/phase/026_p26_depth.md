# Phase: P26 — depth_top matches the book

### Scope

- **Purpose**: Prove the top-N depth view agrees with the book's real state.
- **Responsibility**: `depth_top(2)` returns exactly what the book's top two levels hold.

### Statement

The twenty-sixth phase's one new contract: `depth_top(2)` returns a view that matches the book exactly, without the caller having to walk every resting order itself.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:807` | Phase P26 in the source's Prompt 5 answer for workstream 002 |
