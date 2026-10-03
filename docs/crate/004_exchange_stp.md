# Crate: exchange_stp

**Design status**: Built as its own real crate — see
[`../../module/exchange_stp/readme.md`](../../module/exchange_stp/readme.md).

(Superseded note: this file previously said "Folded into `exchange_match`";
`SelfMatchPolicy` has since moved out to this standalone crate,
`exchange_stp`, with `exchange_match` now re-exporting it instead of owning
it. `SelfMatchCancellation` — the match-outcome record, not pure policy
vocabulary — stayed in `exchange_match`.)

Purpose/boundary/dependency content moved to
[`../../module/exchange_stp/docs/item/readme.md`](../../module/exchange_stp/docs/item/readme.md).
Primary source: `../../../../codename_space_sandbox/intake/core_exchange.txt:374-379`
(Crate 4 in Prompt 2) and `:550-552` (Prompt 3 exposed-item list).
