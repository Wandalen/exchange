# Pitfall: Saturating add hiding an insolvent hold

**Relocated.** This pitfall is single-crate-relevant (`exchange_escrow`'s own
`checked_*` arithmetic choice) and now lives at
[`../../module/exchange_escrow/docs/pitfall/001_saturating_add_hiding_insolvent_hold.md`](../../module/exchange_escrow/docs/pitfall/001_saturating_add_hiding_insolvent_hold.md),
with the "how this crate avoids it" verification alongside it. The other
three "Escrow" pitfalls (011–013) stay here — each describes a call-sequence
invariant spanning this crate and whichever crate calls it, not a choice
internal to this crate alone. (014 has since also relocated to
`exchange_escrow`'s own `docs/pitfall/002_*.md` — see 014's own stub here.)

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:910` | Pitfall in the source's Prompt 7 "Escrow" list |
