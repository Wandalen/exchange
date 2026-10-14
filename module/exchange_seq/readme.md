# exchange_seq

A monotonic sequence for time priority, without a wall clock — a root of the
dependency tree, and not workstream 004's clock. Closes feature 28, which hard
problem 6's determinism depends on: a clock reading differs across replays and
nodes.

```rust
use exchange_seq::{ Sequence, seq_next };

let first = seq_next( Sequence::ZERO );
let second = seq_next( first );
assert!( second > first );
```

## Not built: `seq_cmp`, `SeqError::Exhausted`

`Sequence` already derives `Ord`, and a `u64` bumped once per event does not
run out in any real run — see
[`docs/decisions/001_no_seq_cmp_or_seq_error.md`](docs/decisions/001_no_seq_cmp_or_seq_error.md).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `Sequence` and the one increment, `seq_next` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why `seq_cmp` and `SeqError` are not part of this build |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_seq_test.rs`](tests/exchange_seq_test.rs) | Test Matrix T01 — monotonicity, plus Phase P08's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_core/`](../exchange_core/readme.md) — calls `seq_next` once per emitted event
- [`exchange_book/`](../exchange_book/readme.md) — breaks a same-price tie by arrival `Sequence`
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p08_seq`, this crate's phase smoke
