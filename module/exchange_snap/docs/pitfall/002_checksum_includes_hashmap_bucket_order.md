# Pitfall: Checksum that includes HashMap bucket order

### Scope

- **Purpose**: Name a specific mistake a later consumer of this crate's output must avoid.
- **Responsibility**: Computing a determinism checksum over `BookSnap` data whose own iteration order depends on a hash map's internal layout.
- **In Scope**: Any later checksum computed over `BookSnap::rows`.

### Statement

A determinism checksum is only meaningful if it is itself deterministic —
folding in a hash map's bucket order means the checksum can differ between
two runs of identical input for a reason that has nothing to do with whether
the actual trading outcome was the same, defeating the two-run equality check
the design relies on. This crate doesn't compute a checksum itself (that's a
later consumer's job, likely the P30 wall smoke's `a==b` check), but it owns
the one input such a checksum would be built over.

### How this crate avoids contributing to it

`exchange_book::Book`'s own representation is sorted levels per side, never a
`HashMap`; `snap_take` reads it via `Book::iter()`, so `BookSnap::rows`
inherits the same hash-free, deterministic order — bids best-first, then
asks best-first, each side still ranked by the book's own price-time
priority. A checksum built over `BookSnap::rows` later is sound on this
count without that later code having to re-establish it. Verified directly:
[`tests/exchange_snap_test.rs`](../../tests/exchange_snap_test.rs)'s
`rows_preserve_the_books_priority_order` asserts the exact published
sequence, not merely that the right rows are present.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/031_checksum_includes_hashmap_bucket_order.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:934` | Pitfall in the source's Prompt 7 "Snapshot" list |
