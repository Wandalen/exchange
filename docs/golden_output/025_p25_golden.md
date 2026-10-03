# Golden Output: P25

### Scope

- **Purpose**: The exact pass/fail line `demo_p25_halt` must print.
- **Responsibility**: `halt=1 resume=1 → ok`

### Statement

`demo_p25_halt` prints `halt=1 resume=1` followed by `ok`. The literal line
is unchanged from the source transcript, but what it confirms is narrower:
`exchange_halt` isn't wired into any placement path yet (that belongs to
whichever stage reworks `exchange_match`/`exchange_rest` — see
`exchange_halt`'s own `docs/decisions/001_no_exchange_book_dependency.md`),
so this phase confirms the halt/resume round-trip `exchange_halt` actually
owns today — `halt_set` takes effect (`halt=1`) and `halt_clear` takes it
back (`resume=1`) — not that order placement itself was blocked.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:869` | P25's golden line in the source's Prompt 6 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1216` | Consolidated in Prompt 9's `golden_output` list |
