# Golden Output: P27

### Scope

- **Purpose**: The exact pass/fail line `demo_p27_snap` must print.
- **Responsibility**: `snap=1 live=0 → ok`

### Statement

`demo_p27_snap` prints `snap=1 live=0` confirming the snapshot retained the row the live cancel removed, followed by `ok`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:873` | P27's golden line in the source's Prompt 6 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1216` | Consolidated in Prompt 9's `golden_output` list |
