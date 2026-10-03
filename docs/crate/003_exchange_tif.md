# Crate: exchange_tif

**Design status**: Built as its own real crate, matching the proposal
exactly — see
[`../../module/exchange_tif/docs/item/readme.md`](../../module/exchange_tif/docs/item/readme.md).

(Superseded note: this file previously said "Not built", quoting a
now-outdated `exchange_types` doc comment. `Tif`/`tif_rests`/
`tif_requires_full` all exist; `tif_requires_full` is even consulted and
tested in `exchange_match::cross` for FOK. The remaining gap is
`exchange_core::Exchange::submit` hardcoding `Tif::Gtc` — caller-side
wiring, not a gap in this crate.)

Purpose/boundary/dependency content moved to
[`../../module/exchange_tif/readme.md`](../../module/exchange_tif/readme.md).
Primary source: `../../../../codename_space_sandbox/intake/core_exchange.txt:368-373`
(Crate 3 in Prompt 2) and `:546-548` (Prompt 3 exposed-item list).
