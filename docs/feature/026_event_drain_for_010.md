# Feature: Event Drain For 010

### Scope

- **Purpose**: Hand workstream 010 what happened, once.
- **Responsibility**: Drain fills, rejects, and cancel-acks for the consumer to settle.

### Statement

An event drain hands workstream 010 every fill, reject, and cancel-ack produced this call, exactly once — the one channel through which this workstream's state changes reach the economy layer.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:334` | Feature 26 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1057` | Feature 26's English title, Prompt 9 |
