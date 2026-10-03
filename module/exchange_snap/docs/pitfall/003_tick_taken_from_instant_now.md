# Pitfall: Tick taken from Instant::now()

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Stamping a snapshot's tick from a live clock read instead of accepting it from the caller.
- **In Scope**: `snap_take`'s `tick` parameter.

### Statement

The simulation tick is the caller's concept, not the exchange's own —
reading `Instant::now()` to fill it in ties the snapshot to wall-clock time,
reintroducing the same cross-machine, non-reproducible behavior the whole
design otherwise avoids by keeping priority and sequencing off the clock
entirely (the same reasoning `exchange_book`'s own module doc gives for using
[`exchange_seq::Sequence`] instead of a timestamp).

### How this crate avoids it

`snap_take( book : &Book, instrument : InstrumentId, tick : Money )` takes
`tick` as a plain parameter, supplied by the caller — nothing in this
crate's source imports `std::time` or calls any clock. Grep-verifiable:

```bash
cd module/exchange_snap && command grep -n 'Instant::now()\|SystemTime::now()' src/lib.rs
```

**Expected:** exactly one hit, inside the module doc comment's own prose
*naming* this pitfall (the line you are reading now) — never inside a
function body. A second hit, or any hit inside `fn snap_take`, would mean
the pitfall had actually been reintroduced.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/032_tick_taken_from_instant_now.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:935` | Pitfall in the source's Prompt 7 "Snapshot" list |
