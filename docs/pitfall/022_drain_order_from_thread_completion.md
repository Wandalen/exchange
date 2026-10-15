# Pitfall: Drain order taken from thread completion, not drain_order

**Redistributed**: this pitfall is `exchange_inbound`'s own concern (its
`inbound_ring`/`inbound_drain` are the building blocks the fixed-order
combine is built on, and `docs/decisions/001_two_producers_is_two_rings.md`
records the design decision directly) — see
[`../../module/exchange_inbound/docs/pitfall/001_drain_order_from_thread_completion.md`](../../module/exchange_inbound/docs/pitfall/001_drain_order_from_thread_completion.md)
for the full writeup, including the structural guarantee and the
cross-crate two-thread demonstration (`smoke_exchange_phases`'s
`demo_p28_drain.rs`) it rests on.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:921` | Pitfall in the source's Prompt 7 "Ring" list |
