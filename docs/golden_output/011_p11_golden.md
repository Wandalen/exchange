# Golden Output: P11

### Scope

- **Purpose**: The exact pass/fail line `demo_p11_walk` must print.
- **Responsibility**: `w=1,0.95 → ok`

### Statement

`demo_p11_walk` prints `w=1,0.95` confirming the price walk follows sorted order, followed by `ok`.

**Adapted from the source transcript's own `w=1.00,0.95`** (confirmed built and
run 2026-10-03): this family's real `exact_kind::Decimal::fmt` trims an
exactly-zero fraction to nothing, decimal point included, so the whole-number
price `1.00` prints as `1`, never `1.00` — `0.95`'s non-zero fraction is
unaffected. See `docs/golden_output/026_p26_golden.md` for the same adaptation
on the same formatter.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:841` | P11's golden line in the source's Prompt 6 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1213` | Consolidated in Prompt 9's `golden_output` list |
