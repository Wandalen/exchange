# Golden Output: P20

### Scope

- **Purpose**: The exact pass/fail line `demo_p20_partial` must print.
- **Responsibility**: `fills=10,2 rest=3 → ok`

### Statement

`demo_p20_partial` prints `fills=10,2 rest=3` confirming the taker-for-12 split correctly across two makers, followed by `ok`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:859` | P20's golden line in the source's Prompt 6 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1215` | Consolidated in Prompt 9's `golden_output` list |
