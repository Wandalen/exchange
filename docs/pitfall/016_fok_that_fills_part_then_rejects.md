# Pitfall: FOK that fills part, then rejects

**Relocated.** This pitfall is single-crate-relevant (`exchange_match`'s own
probe-then-commit mechanism) and now lives at
[`../../module/exchange_match/docs/pitfall/001_fok_that_fills_part_then_rejects.md`](../../module/exchange_match/docs/pitfall/001_fok_that_fills_part_then_rejects.md),
with the "how this crate avoids it" verification alongside it.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:913` | Pitfall in the source's Prompt 7 "Match policy" list |
