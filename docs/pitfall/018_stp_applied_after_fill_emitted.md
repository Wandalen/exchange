# Pitfall: STP applied after the fill is already emitted

**Relocated.** This pitfall is single-crate-relevant (`exchange_match`'s
own `cross_inner` ordering — the self-match check runs before any `Trade`
is built, not after) and now lives at
[`../../module/exchange_match/docs/pitfall/002_stp_applied_after_fill_emitted.md`](../../module/exchange_match/docs/pitfall/002_stp_applied_after_fill_emitted.md),
with the "how this crate avoids it" verification alongside it.
`exchange_stp` holds only the policy enum, so the ordering is
`exchange_match`'s alone.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:915` | Pitfall in the source's Prompt 7 "Match policy" list |
