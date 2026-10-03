# exchange_stats

Running counters for the hot path — rests, fills, rejects, and cancels — so a caller
can read what the match loop has done instead of re-scanning the event log
for it. A root of the dependency tree.

```rust
use exchange_stats::{ stats_fill_add, stats_snapshot, stats_zero };

let mut stats = stats_zero();
stats_fill_add( &mut stats, 2 );
assert_eq!( stats_snapshot( &stats ).fills, 2 );
```

## Genuinely new

No running counter exists anywhere in the real crates today — the only
record of what happened is the full `Event` log. This crate closes hard
problem 14 (hot path) and feature 23 (`BookStats`).

## Diverges from the proposal

The source design names `exchange_id` as a dependency. Nothing here is keyed
by any identity — every counter is a plain running total — so the dependency
is not taken; see [`docs/decisions/001_no_exchange_id_dependency.md`](docs/decisions/001_no_exchange_id_dependency.md).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `BookStats` and its six counter/snapshot functions |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why `exchange_id` is not a dependency |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_stats_test.rs`](tests/exchange_stats_test.rs) | Test Matrix T01 — accumulation and snapshot independence |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |
