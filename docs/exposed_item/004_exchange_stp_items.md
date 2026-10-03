# Exposed Item: exchange_stp

**Design status**: Built as its own real crate — see
[`../../module/exchange_stp/docs/item/readme.md`](../../module/exchange_stp/docs/item/readme.md)
for the full as-built-vs-proposed listing, and
[`../../module/exchange_stp/docs/decisions/001_no_allow_resting_incoming_naming.md`](../../module/exchange_stp/docs/decisions/001_no_allow_resting_incoming_naming.md)
for the no-`Allow`/renaming/`CancelBoth` reasoning.

(Superseded note: this file previously claimed "No `stp_name` function
exists (the enum derives `Debug` instead))" — false now. `stp_name` is
built exactly as proposed, and tested
(`module/exchange_stp/tests/exchange_stp_test.rs`). The owning crate also
moved — `SelfMatchPolicy` now lives in `exchange_stp`, not
`exchange_match`. The `exchange_core` hardcoding claim is still real, at
updated line numbers: `module/exchange_core/src/lib.rs:62,316,338`.)

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:550-552` | Crate `exchange_stp`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
