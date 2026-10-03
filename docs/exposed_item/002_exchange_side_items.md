# Exposed Item: exchange_side

**Design status**: Built as its own real crate, matching the proposal's
full named function list (`side_opposite`, `side_is_bid`, `side_is_ask`
all exist). (Superseded note: this file previously claimed `side_is_bid`/
`side_is_ask` didn't exist — they were added since. Only the enum variant
naming still diverges: `Buy`/`Sell`, not `Bid`/`Ask`.)

Full, verified exposed-surface listing moved to
[`../../module/exchange_side/docs/item/readme.md`](../../module/exchange_side/docs/item/readme.md).
Why the variants are `Buy`/`Sell`:
[`../../module/exchange_side/docs/decisions/001_buy_sell_not_bid_ask.md`](../../module/exchange_side/docs/decisions/001_buy_sell_not_bid_ask.md).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:542-544` | Crate `exchange_side`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
