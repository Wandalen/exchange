# Pitfall: Ring overflow as a silent drop

**Redistributed**: this pitfall is `exchange_inbound`'s own concern (its
`inbound_ring` is the only place `OverflowPolicy` is chosen, and
`inbound_overflow_reject` is the only place a full ring's rejection
surfaces) — see
[`../../module/exchange_inbound/docs/pitfall/002_ring_overflow_as_silent_drop.md`](../../module/exchange_inbound/docs/pitfall/002_ring_overflow_as_silent_drop.md)
for the full writeup and the test that verifies the avoidance.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:923` | Pitfall in the source's Prompt 7 "Ring" list |
