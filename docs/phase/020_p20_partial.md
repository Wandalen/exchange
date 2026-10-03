# Phase: P20 — Partial fill across two makers

### Scope

- **Purpose**: Prove a taker that outsizes one maker fills against a second and leaves the right remainder.
- **Responsibility**: An ask for 12 against bids of 10 then 5 at the same price fills 10 and 2, leaving 3.

### Statement

The twentieth phase's one new contract: a taker order for 12 crossing a resting 10 and then a resting 5 (same price) produces fills of 10 and 2, with 3 left resting from the second maker — partial fills that split correctly across multiple makers.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:801` | Phase P20 in the source's Prompt 5 answer for workstream 002 |
