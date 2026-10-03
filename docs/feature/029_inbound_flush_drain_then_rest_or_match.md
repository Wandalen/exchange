# Feature: Inbound Flush, Drain, Then Rest Or Match

### Scope

- **Purpose**: The fixed sequence an incoming order follows before it touches the book.
- **Responsibility**: Flush thread-local staging, drain the ring in total order, then rest or match.

### Statement

An incoming order's path is fixed: thread-local staging flushes into the ring, the ring drains in total order, and only then does an individual order rest or match — never matched straight off a producer thread.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:337` | Feature 29 in the source's Prompt 1 answer for workstream 002 |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1060` | Feature 29's English title, Prompt 9 |
