# Pitfall: ring_spsc as the market path (many producers)

**Redistributed**: this pitfall is `exchange_inbound`'s own concern (its
`inbound_ring` is the only place a ring's producer cardinality is decided
for the market path) — see
[`../../module/exchange_inbound/docs/pitfall/003_ring_spsc_as_market_path.md`](../../module/exchange_inbound/docs/pitfall/003_ring_spsc_as_market_path.md)
for the full writeup, including why the real rings built here genuinely are
`ring_spsc`-backed and why that is still the correct avoidance, not a miss.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:924` | Pitfall in the source's Prompt 7 "Ring" list |
