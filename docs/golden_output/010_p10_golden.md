# Golden Output: P10

### Scope

- **Purpose**: The exact pass/fail line `demo_p10_best` must print.
- **Responsibility**: `best=1 → ok`

### Statement

`demo_p10_best` prints `best=1` confirming the correct best bid after two inserts, followed by `ok`.

**Adapted from the source transcript's own `best=1.00`** (confirmed built and
run 2026-10-03): this family's real `exact_kind::Decimal::fmt` trims an
exactly-zero fraction to nothing, decimal point included, so the whole-number
price `1.00` prints as `1`, never `1.00` — see `docs/golden_output/026_p26_golden.md`
for the same adaptation on the same formatter.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:839` | P10's golden line in the source's Prompt 6 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1213` | Consolidated in Prompt 9's `golden_output` list |
