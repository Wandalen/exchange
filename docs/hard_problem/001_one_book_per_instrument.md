# Hard Problem: One Book Per Instrument

**Relocated.** This hard problem is single-crate-relevant (`exchange_book`'s
own per-instrument storage) and now lives at
[`../../module/exchange_book/readme.md`](../../module/exchange_book/readme.md):
`Book` keys every side by `InstrumentId`, which `exchange_order::Order`
carries.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:187-190` | Hard problem 1 in the source's Prompt 1 answer for workstream 002 |
