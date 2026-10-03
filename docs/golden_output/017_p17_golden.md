# Golden Output: P17

### Scope

- **Purpose**: The exact pass/fail line `demo_p17_cons` must print.
- **Responsibility**: `z=1 nz=1 → ok`

### Statement

`demo_p17_cons` prints `z=1 nz=1` confirming an ordinary batch conserves to zero (`z`) and a trade whose own notional is inexact is correctly refused (`nz`), followed by `ok`.

Revised from the original 10/-10-vs-10/-9 magnitude-mismatch framing: wiring
`conserve_assert` against a real `exchange_match::cross` batch (Stage 6)
proved that shape unreachable, since both of a trade's legs go in together —
see `exchange_conserve/src/lib.rs`'s "Revision" section.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:853` | P17's golden line in the source's Prompt 6 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1214` | Consolidated in Prompt 9's `golden_output` list |
