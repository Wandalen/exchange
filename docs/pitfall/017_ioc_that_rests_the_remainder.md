# Pitfall: IOC that rests the remainder

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Leaving an Immediate-or-Cancel order's unfilled quantity resting on the book.
- **In Scope**: Match policy

**Not `exchange_match`'s own.** That crate's own module doc
(`module/exchange_match/src/lib.rs`, "Time-in-force" section) states
directly: "IOC needs no logic here — this function has never inserted the
incoming order's remainder itself... only whether a *caller* later rests
that remainder differs, which is `exchange_rest`/`exchange_core`'s decision,
not this crate's." The real owner is whichever crate decides to rest (or
not) — `exchange_core`, or the very new `exchange_rest` crate it may be
extracting into (both actively changing as of 2026-10-03). Deferred rather
than assigned to either while that boundary is still moving; stays central
for now.

### Statement

IOC means take whatever is immediately available and discard the rest — an implementation that falls through to the normal "rest the remainder" path for an IOC order silently turns it into a GTC order, which is a different time-in-force than the one the caller asked for.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:914` | Pitfall in the source's Prompt 7 "Match policy" list |
