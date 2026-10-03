# Pitfall: HashMap iteration as price order

**Relocated.** This pitfall is single-crate-relevant (`exchange_book`'s own
`bids`/`asks` storage choice) and now lives at
[`../../module/exchange_book/docs/pitfall/001_hashmap_iteration_as_price_order.md`](../../module/exchange_book/docs/pitfall/001_hashmap_iteration_as_price_order.md),
with the "how this crate avoids it" verification alongside it. The other
five "Book" pitfalls (006–010) stay here for now — they describe
arrival/FIFO ordering within one price level, which is `exchange_level`'s
own storage, not this crate's; pending that crate's own redistribution pass.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:898` | Pitfall in the source's Prompt 7 "Book" list |
