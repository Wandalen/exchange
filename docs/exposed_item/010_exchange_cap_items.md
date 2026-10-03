# Exposed Item: exchange_cap

**Design status**: Built as its own real crate, matching the proposal
exactly — see
[`../../module/exchange_cap/docs/item/readme.md`](../../module/exchange_cap/docs/item/readme.md)
for the full as-built-vs-proposed listing.

(Superseded note: this file previously claimed "Not built — no real crate
exists for this" — false now. `BookCaps`, `cap_check_rest`,
`cap_check_level`, and `CapError` all exist exactly as proposed; see the
pitfall redistribution at
[`../../module/exchange_cap/docs/pitfall/001_full_drops_order_returns_ok.md`](../../module/exchange_cap/docs/pitfall/001_full_drops_order_returns_ok.md)
for how the `Full`-is-an-error guarantee is verified. `exchange_book`'s own
Vec-growth not yet consulting the cap is caller-side wiring, not a gap in
this crate.)

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:587-590` | Crate `exchange_cap`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
