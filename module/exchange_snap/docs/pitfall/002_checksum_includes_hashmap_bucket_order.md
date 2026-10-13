# Pitfall: Checksum that includes HashMap bucket order

### Scope

- **Purpose**: Name a specific mistake a later consumer of this crate's output must avoid.
- **Responsibility**: Computing a determinism checksum over `BookSnap` data whose own iteration order depends on a hash map's internal layout.
- **In Scope**: Any later checksum computed over `BookSnap::rows`.

### Statement

A determinism checksum is only meaningful if it is itself deterministic.
Folding in a hash map's bucket order lets two runs of identical input differ
for a reason unrelated to the trading outcome, defeating the two-run equality
check. This crate computes no checksum, but it owns the rows one would be
built over.

### How this crate avoids contributing to it

`exchange_book::Book` keeps sorted levels per side, never a `HashMap`, and
`snap_take` reads it through `Book::side`, so `BookSnap::rows` comes out in
the book's own order: bids best first, then asks best first.
[`tests/exchange_snap_test.rs`](../../tests/exchange_snap_test.rs)'s
`rows_preserve_the_books_priority_order` asserts the exact sequence, not only
that the right rows are present.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/031_checksum_includes_hashmap_bucket_order.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:934` | Pitfall in the source's Prompt 7 "Snapshot" list |
