# exchange_stats

Running counters for the hot path — rests, fills, rejects, cancels — so a
caller reads what the match loop did instead of re-scanning the event log. A
root of the dependency tree. Closes hard problem 14 and feature 23.

```rust
use exchange_stats::{ stats_fill_add, stats_snapshot, stats_zero };

let mut stats = stats_zero();
stats_fill_add( &mut stats, 2 );
assert_eq!( stats_snapshot( &stats ).fills, 2 );
```

## Wired into `exchange_core`

`step_place`/`step_one` bump the counters on every step;
`Exchange::stats_get` returns a snapshot.

## Diverges from the proposal

- No `exchange_id` dependency: nothing here is keyed by an id — see
  [`docs/decisions/001_no_exchange_id_dependency.md`](docs/decisions/001_no_exchange_id_dependency.md).
- `stats_rest_add`/`stats_cancel_add` are added: the proposal's function list
  leaves `rests` and `cancels` with no incrementer.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `BookStats` and its six counter/snapshot functions |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why `exchange_id` is not a dependency |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_stats_test.rs`](tests/exchange_stats_test.rs) | Test Matrix T01 — accumulation and snapshot independence |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_core/`](../exchange_core/readme.md) — the real caller, via `step_place`/`step_one`/`stats_get`
