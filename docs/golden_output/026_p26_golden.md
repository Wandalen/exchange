# Golden Output: P26

### Scope

- **Purpose**: The exact pass/fail line `demo_p26_depth` must print.
- **Responsibility**: `d=1:3,0.95:4 → ok`

### Statement

`demo_p26_depth` prints `d=1:3,0.95:4` confirming `depth_top(2)` matched the book exactly, followed by `ok`.

**Adapted from the source transcript's own `d=1.00:3,0.95:4`** (confirmed built
and run 2026-10-02): this family's real `exact_kind::Decimal::fmt` trims an
exactly-zero fraction to nothing, decimal point included, so the whole-number
price `1.00` prints as `1`, never `1.00` — `0.95`'s non-zero fraction is
unaffected. Same book, same `depth_top` result; only one price's literal
zero-padding differs from the transcript's hand-written example.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:871` | P26's golden line in the source's Prompt 6 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1216` | Consolidated in Prompt 9's `golden_output` list |
