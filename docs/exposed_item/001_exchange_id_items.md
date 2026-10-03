# Exposed Item: exchange_id

**Design status**: Built as its own real crate, matching the proposal's
full named item list except `IdError`. (Superseded note: this file
previously described a pre-Stage-1 state where everything was folded into
`exchange_types` with no `InstrumentId` and no conversion functions —
Stage 1's real build resolved all of that; only the `IdError` gap is
still real.)

Full, verified exposed-surface listing moved to
[`../../module/exchange_id/docs/item/readme.md`](../../module/exchange_id/docs/item/readme.md).
Why there is no `IdError`:
[`../../module/exchange_id/docs/decisions/001_no_id_error.md`](../../module/exchange_id/docs/decisions/001_no_id_error.md).

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:535-540` | Crate `exchange_id`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
