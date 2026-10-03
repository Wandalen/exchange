# Golden Output: P28

### Scope

- **Purpose**: The exact pass/fail line `demo_p28_drain` must print.
- **Responsibility**: `a=0x… b=0x… → ok if a==b`

### Statement

`demo_p28_drain` prints two run checksums, `a` and `b`; the phase passes only if they are equal, confirming deterministic drain order across runs.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:875` | P28's golden line in the source's Prompt 6 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1216` | Consolidated in Prompt 9's `golden_output` list |
