# Feature: Book

### Scope

- **Purpose**: One book per instrument.
- **Responsibility**: Hold one instrument's resting bid and ask ladders.

### Statement

A `Book` holds one instrument's resting orders, sorted into its bid and ask ladders — the central structure every other operation in this workstream reads or mutates.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:314` | Feature 6 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1037` | Feature 6's English title, Prompt 9 |
