# Golden Output: P24

### Scope

- **Purpose**: The exact pass/fail line `demo_p24_stp` must print.
- **Responsibility**: `self=0 → ok`

### Statement

`demo_p24_stp` prints `self=0` confirming zero self-fills occurred under `CancelResting` — see `../phase/024_p24_stp.md` for why this is the real policy name, not the source proposal's `CancelOldest`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:867` | P24's golden line in the source's Prompt 6 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1216` | Consolidated in Prompt 9's `golden_output` list |
