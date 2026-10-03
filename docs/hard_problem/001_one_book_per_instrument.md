# Hard Problem: One Book Per Instrument

**Relocated.** This hard problem is single-crate-relevant (`exchange_book`'s
own per-instrument storage) and now lives at
[`../../module/exchange_book/readme.md`](../../module/exchange_book/readme.md)'s
"Closes hard problem 1, and features 6 and 22" section. (The central Design
status this entry used to carry — "Not addressed... the real book is
implicitly single-instrument" — is stale: `exchange_book::Book` now keys
every side by `InstrumentId`, and `exchange_order::Order` now carries one.)

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:187-190` | Hard problem 1 in the source's Prompt 1 answer for workstream 002 |
