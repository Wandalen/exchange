# Golden Output: P14

### Scope

- **Purpose**: The exact pass/fail line `demo_p14_hold` must print.
- **Responsibility**: `rel=1 com=1 → ok`

### Statement

`demo_p14_hold` prints `rel=1 com=1` confirming cancel released the hold and fill committed it, followed by `ok`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:847` | P14's golden line in the source's Prompt 6 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1214` | Consolidated in Prompt 9's `golden_output` list |
