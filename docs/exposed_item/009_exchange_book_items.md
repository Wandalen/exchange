# Exposed Item: exchange_book

### Scope

- **Purpose**: The full exposed surface Prompt 3 specifies for `exchange_book`.
- **Responsibility**: The sorted bid/ask ladders for one instrument.

**Design status**: Built as `Book`/`Resting`: one `Book` holds every
instrument, keyed inside, with `best( instrument, side )`, `side`, `iter`,
`len`/`is_empty` and the cap counts — no `book_*` free functions, no
`book_level_get`, no `BookError`. Built-vs-proposed comparison:
[`../../module/exchange_book/docs/item/readme.md`](../../module/exchange_book/docs/item/readme.md).

### Statement

Prompt 3 specifies one `Book` scoped to one instrument, keyed externally by the caller, with seven free functions and a `BookError`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:579-585` | Crate `exchange_book`'s exposed-item list in the source's Prompt 3 answer |
| `../../../../codename_space_sandbox/intake/core_exchange.txt:1238-1239` | Same items also appear in Prompt 9's flat `exposed_item` dump |
