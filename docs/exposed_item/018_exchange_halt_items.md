# Exposed Item: exchange_halt

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_halt`.
- **Responsibility**: Freezing the match loop while resting orders remain untouched.

**Design status**: Built as listed — `halt_set`/`halt_clear`/`halt_is`, `HaltError { Already }`. Built-vs-proposed comparison: [`../../module/exchange_halt/docs/item/readme.md`](../../module/exchange_halt/docs/item/readme.md).

### Statement

Prompt 3 specifies a three-function halt/resume toggle with its own error type.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:631-633` | Crate `exchange_halt`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
