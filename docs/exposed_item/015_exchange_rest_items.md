# Exposed Item: exchange_rest

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_rest`.
- **Responsibility**: Placing, cancelling, and replacing a resting order.

**Design status**: Built as `rest_place`/`rest_cancel` — forwards to
`Book::insert`/`Book::cancel`, returning their `bool`/`Option<Resting>` —
and `rest_replace` with its own `RestReplaceError`. No `RestError`: a
duplicate is `Book::insert`'s refusal, and caps, halt and escrow are checked
by the caller (`RejectReason`, `ExchangeError`). Built-vs-proposed comparison:
[`../../module/exchange_rest/docs/item/readme.md`](../../module/exchange_rest/docs/item/readme.md).

### Statement

Prompt 3 specifies a `rest_place`/`rest_cancel`/`rest_replace` trio behind
one `RestError { Duplicate, Missing, Full, Halted, Escrow, Snap }`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:615-617` | Crate `exchange_rest`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
