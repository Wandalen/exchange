# exchange_cap

A limit on how large a book may grow, with a named refusal past it — a root of
the dependency tree, and this crate does not itself match.

```rust
use exchange_cap::{ BookCaps, CapError, cap_check_rest };

let caps = BookCaps { max_rests : 2, max_levels : 2 };
assert_eq!( cap_check_rest( caps, 2 ), Err( CapError::RestsFull ) );
```

## Genuinely new

No capacity limit exists anywhere in the real crates today — a book either
grows without bound or silently drops an order nothing else reports as
missing. This crate closes hard problem 17 and feature 20. Nothing yet calls
[`cap_check_rest`]/[`cap_check_level`] from `exchange_book` — that wiring is a
later stage.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `BookCaps`, `CapError`, and the two cap checks |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | The 1 "Identity and cap" pitfall this crate's return value bears on |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/exchange_cap_test.rs`](tests/exchange_cap_test.rs) | Test Matrix T01 — the cap check, plus Phase P12's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p12_cap`, this crate's phase smoke
