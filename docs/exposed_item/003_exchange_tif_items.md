# Exposed Item: exchange_tif

**Design status**: Built as its own real crate, matching the proposal
exactly — see
[`../../module/exchange_tif/docs/item/readme.md`](../../module/exchange_tif/docs/item/readme.md)
for the full as-built-vs-proposed listing.

(Superseded note: this file previously claimed "Not built — no real crate
exists for this" and "every order here behaves as GTC" — both stale.
`Tif { Gtc, Ioc, Fok }`, `tif_rests`, and `tif_requires_full` all exist
exactly as proposed. `tif_requires_full` is consulted and tested in
`exchange_match::cross` for FOK
([`../../module/exchange_tif/docs/tif/003_fok.md`](../../module/exchange_tif/docs/tif/003_fok.md)).
Every order still behaves as GTC in practice, but because
`exchange_core::Exchange::submit` hardcodes `Tif::Gtc`, not because the
other values are unimplemented.)

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:546-548` | Crate `exchange_tif`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
