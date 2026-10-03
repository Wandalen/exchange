# exchange_seq

A monotonic sequence for time priority, without a wall clock — a root of the
dependency tree, and not workstream 004's clock.

```rust
use exchange_seq::{ Sequence, seq_next };

let first = seq_next( Sequence::ZERO );
let second = seq_next( first );
assert!( second > first );
```

## Extracted from `exchange_types`

`Sequence` lived in `exchange_types` until this crate split out;
`exchange_types` now depends on this crate and re-exports it, so every
existing `use exchange_types::Sequence` still resolves. [`seq_next`] is new:
it gives the increment a name and a home, replacing the raw `u64` counter
`exchange_core` previously incremented by hand — `Exchange::emit`
(`module/exchange_core/src/lib.rs`) now calls it instead.

## Not built: `SeqError::Exhausted`, `seq_cmp`

Both are named in the source design; neither is built here. See
[`docs/decisions/001_no_seq_cmp_or_seq_error.md`](docs/decisions/001_no_seq_cmp_or_seq_error.md)
for why.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `Sequence` and the one increment, `seq_next` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why `seq_cmp` and `SeqError` are not part of this build |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_seq_test.rs`](tests/exchange_seq_test.rs) | Test Matrix T01 — monotonicity, plus Phase P08's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_types/`](../exchange_types/readme.md) — re-exports `Sequence` for existing callers
- [`exchange_core/`](../exchange_core/readme.md) — calls `seq_next` once per emitted event
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p08_seq`, this crate's phase smoke
