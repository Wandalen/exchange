# Exposed Item: exchange_match

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_match`.
- **Responsibility**: Crossing a taker against the opposite ladder, with partials, TIF, and STP.

**Design status**: Built as one `cross( book, incoming, policy )` returning
`Crossing { trades, remaining, cancelled }`: partial fills, self-match
prevention and the FOK probe run inside it, and there is no `rejects` —
turning an outcome into a rejection is `exchange_core`'s. `MatchError` is
`{ Quantity, BookDesynchronized, Conservation, PostOnlyWouldTake }`.
Built-vs-proposed comparison:
[`../../module/exchange_match/docs/item/readme.md`](../../module/exchange_match/docs/item/readme.md).

### Statement

Prompt 3 specifies `MatchOut { fills, rest, rejects }`, four phase functions
— `match_in`, `match_partial`, `match_fok_check`, `match_stp_apply` — and
`MatchError { Halted, Escrow, Conserve, Empty }`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:619-623` | Crate `exchange_match`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
