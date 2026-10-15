# Pitfall: IOC that rests the remainder

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Leaving an Immediate-or-Cancel order's unfilled quantity resting on the book.
- **In Scope**: Match policy

**Not `exchange_match`'s own**: `cross` never rests the incoming order.
Each caller that rests a remainder decides, and both ask
`exchange_tif::tif_rests` first — `exchange_core::step_place`, pinned by
`an_ioc_taker_never_rests_its_remainder_through_the_facade`, and
`exchange_inbound::inbound_apply`. `exchange_rest::rest_place` only seats
what it is handed. Stays central: no single crate owns it.

### Statement

IOC means take whatever is immediately available and discard the rest — an implementation that falls through to the normal "rest the remainder" path for an IOC order silently turns it into a GTC order, which is a different time-in-force than the one the caller asked for.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:914` | Pitfall in the source's Prompt 7 "Match policy" list |
