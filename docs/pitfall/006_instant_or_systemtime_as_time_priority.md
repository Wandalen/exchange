# Pitfall: Instant or SystemTime as time priority

### Scope

- **Purpose**: Name a specific mistake to avoid while building workstream 002.
- **Responsibility**: Using a wall-clock reading (`Instant`/`SystemTime`) to break same-price ties instead of a claimed sequence position.
- **In Scope**: Book

**Design status**: Avoided, and explicitly designed against — `grep -n "Instant\|SystemTime"` across `module/exchange_book/src/lib.rs` and `module/exchange_level/src/lib.rs` returns zero hits. The real `arrival : Sequence` field lives on `exchange_level::LevelNode` (which `exchange_book::Resting` type-aliases, not declares — `module/exchange_book/src/lib.rs:86`), and `exchange_book`'s own module doc comment gives the exact reasoning this pitfall warns about: "Ties are broken by `Sequence`, a claimed position, and never by a clock reading... with time priority resolved by timestamp, two same-price orders would be ranked by whichever host's clock was ahead, the result would differ between machines, and every single-threaded test... would still pass."

**Not cleanly `exchange_book`'s own**, for the same reason as pitfalls 007
and 010: the `arrival` field this pitfall is about is declared and consumed
by `exchange_level`, not `exchange_book`. `exchange_level`'s own
redistribution has since landed (its `docs/item/readme.md` confirms
`LevelNode.arrival : Sequence`), which settles rather than reopens this:
the correctness this pitfall names is jointly held — `exchange_level` must
keep storing `Sequence`, not a clock reading, and `exchange_book` must keep
consulting it without introducing a clock read of its own in its own match
loop. A regression in either crate alone reintroduces the pitfall, so it
stays central permanently rather than moving to one owner.

### Statement

A clock reading is not a priority — two orders submitted close together can read the same timestamp, or read different timestamps on different machines for the same logical moment, so tie-breaking on wall-clock time makes the match outcome depend on hardware rather than on arrival order, and the bug stays invisible in any single-threaded test.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:899` | Pitfall in the source's Prompt 7 "Book" list |
