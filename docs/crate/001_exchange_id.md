# Crate: exchange_id

**Design status**: Built as its own real crate — see
[`../../module/exchange_id/readme.md`](../../module/exchange_id/readme.md).
(Superseded note: this file previously said "Folded into `exchange_types`";
that was true before Stage 1's real build, which split `exchange_id` out as
a standalone crate. `exchange_types` depended on it and re-exported
`AccountId`/`OrderId` for a time after that; its own retirement as a
re-export aggregator has since cut every consumer over to `exchange_id`
directly, so `exchange_types` now only reaches it as a dev-dependency, for
its own tests.)

Purpose/boundary/dependency content moved to
[`../../module/exchange_id/docs/item/readme.md`](../../module/exchange_id/docs/item/readme.md)
and
[`../../module/exchange_id/docs/definition/readme.md`](../../module/exchange_id/docs/definition/readme.md).
Primary source: `../../../../codename_space_sandbox/intake/core_exchange.txt:356-361`
(Crate 1 in Prompt 2) and `:535-540` (Prompt 3 exposed-item list).
