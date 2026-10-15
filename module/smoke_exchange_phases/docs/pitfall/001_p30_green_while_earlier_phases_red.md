# Pitfall: P30 green while P20, P23, P28, or P29 are red

### Scope

- **Purpose**: Name a specific mistake this crate's own existence as a separate suite is meant to prevent.
- **Responsibility**: Treating the final wall-smoke pass as sufficient evidence even when an earlier phase smoke has regressed.
- **In Scope**: This crate's own phase binaries, run as a full set.

### Statement

The wall (P30, `smoke_exchange_book`) exercises most of the design at once,
but it is not a superset proof of every earlier phase — partial fills
(P20), FOK-on-thin-book (P23), deterministic drain (P28), and ring overflow
(P29) are specific enough that a regression in one can hide inside a P30
pass that happens to route around it, so CI must keep checking every
earlier phase, not just the last one.

### How this crate avoids it — partially, by existing; not yet by enforcement

This crate's entire reason to exist (see this crate's own
[`readme.md`](../../readme.md)'s "Why a separate crate from the wall smoke")
is exactly this: grading each phase independently rather than only through
the wall. That gives every phase, including P20/P23, a dedicated binary the
wall's own pass can't silently substitute for:

```bash
cd module/smoke_exchange_phases && ls src/bin/*.rs | wc -l
```

**Expected:** `27` (P01–P27; P28/P29 don't exist yet — they're
`exchange_inbound`'s phases, not yet built).

What this crate does **not** do: nothing here forces every binary to
actually be *run* on each change — `cargo build` compiles all of them, but
running the full P01–P27 set (let alone the not-yet-built P28–P30) is a
CI/process step outside this crate's own control. The structural half (a
dedicated binary per phase) is real; the enforcement half (something that
refuses to call the suite green unless every binary ran) isn't built here.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/033_p30_green_while_earlier_phases_red.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:938` | Pitfall in the source's Prompt 7 "Process" list |
