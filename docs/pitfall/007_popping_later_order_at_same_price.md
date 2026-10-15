# Pitfall: Popping the later order at the same price

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Serving a same-price level out of FIFO order.
- **In Scope**: Book

**Not cleanly `exchange_book`'s own.** FIFO order *within* one price level is
`exchange_level`'s own queue (`LevelNode`/`level_pop_front`,
`module/exchange_book/src/lib.rs`'s own `use` list) — `exchange_book`
consumes it but does not implement it. `exchange_level`'s own redistribution
has since landed, confirming this is jointly held rather than resolving to
one owner: `exchange_level::level_pop_front` must keep popping FIFO-correct,
and `exchange_book` must keep calling it rather than iterating a level's
nodes some other way. Stays central permanently.

### Statement

Within one price level, the earlier-arrived order must be served first; popping the later one instead breaks price-time priority silently, since both orders sit at the same price and the bug only shows up as an unfair fill order, never as a crash or a visible error.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:900` | Pitfall in the source's Prompt 7 "Book" list |
