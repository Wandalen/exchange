# Pitfall: Crossing a worse level while a better one still has qty

**Relocated.** This pitfall is single-crate-relevant (`exchange_book`'s own
`best`/`consume_best` indexing, which never reaches any level but the
current front one) and now lives at
[`../../module/exchange_book/docs/pitfall/002_crossing_worse_level_while_better_has_qty.md`](../../module/exchange_book/docs/pitfall/002_crossing_worse_level_while_better_has_qty.md),
with the "how this crate avoids it" verification alongside it.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:901` | Pitfall in the source's Prompt 7 "Book" list |
