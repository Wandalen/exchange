# Pitfall: P30 green while P20, P23, P28, or P29 are red

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Treating the final wall-smoke pass as sufficient evidence even when an earlier phase smoke has regressed.
- **In Scope**: Process

Redistributed to the single crate this testing-process concern belongs to:
[`../../module/smoke_exchange_phases/docs/pitfall/001_p30_green_while_earlier_phases_red.md`](../../module/smoke_exchange_phases/docs/pitfall/001_p30_green_while_earlier_phases_red.md)
— that writeup records what the crate's structure actually covers (a
dedicated binary per phase) versus what it doesn't (nothing forces every
binary to run). This entry stays here only as the central catalog's own
record.

### Statement

The wall (P30) exercises most of the design at once, but it is not a superset proof of every earlier phase — partial fills (P20), FOK-on-thin-book (P23), deterministic drain (P28), and ring overflow (P29) are specific enough that a regression in one can hide inside a P30 pass that happens to route around it, so CI must keep checking every earlier phase, not just the last one.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:938` | Pitfall in the source's Prompt 7 "Process" list |
