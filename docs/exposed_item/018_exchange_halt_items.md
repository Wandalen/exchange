# Exposed Item: exchange_halt

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_halt`.
- **Responsibility**: Freezing the match loop while resting orders remain untouched.

**Design status**: Built, in its own `exchange_halt` crate — this summary predates that extraction. `halt_set`/`halt_clear`/`halt_is`/`HaltError { Already }` all exist, matching the proposal's functions and error shape exactly; the one real divergence is a dropped `exchange_book` dependency. Verified built-vs-proposed comparison: [`../../module/exchange_halt/docs/item/readme.md`](../../module/exchange_halt/docs/item/readme.md).

### Statement

Prompt 3 specifies a three-function halt/resume toggle with its own error type. See the per-crate doc linked above — the real build has exactly this, in its own crate.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:631-633` | Crate `exchange_halt`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
