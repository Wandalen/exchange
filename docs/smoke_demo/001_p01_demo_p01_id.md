# Smoke Demo: demo_p01_id

### Scope

- **Purpose**: Grade phase P01 with a tiny, cheap, golden-line-checkable binary.
- **Responsibility**: Round-trip a raw id 7 out and back, print whether it matches.

### Statement

`demo_p01_id` takes the raw value 7, converts it to `InstrumentId`/`OrderId` and back, and prints whether the round-trip held.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:821` | Phase P01's smoke in the source's Prompt 6 answer for workstream 002 |
