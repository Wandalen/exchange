# exchange_cap

A limit on how large a book may grow, with a named refusal past it — a root of
the dependency tree, and this crate does not itself match. Closes hard problem
17 and feature 20.

```rust
use exchange_cap::{ BookCaps, CapError, cap_check_rest };

let caps = BookCaps { max_rests : 2, max_levels : 2, max_account_rests : 4 };
assert_eq!( cap_check_rest( caps, 2 ), Err( CapError::RestsFull ) );
```

## Wired into `exchange_core`

`Exchange::caps_set` registers caps per instrument; an instrument without caps
is unenforced. `step_place` runs the checks in its dry run, against what would
actually rest, and rejects with `RejectReason::RestsFull`/`LevelsFull`/
`AccountFull`. The level check runs only when the order would open a new price
level.

## Per-account cap

`max_account_rests` limits how many orders one account may rest on an
instrument's book, both sides together — not in the source design. Without it
one account can take every slot the other two caps allow.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `BookCaps`, `CapError`, and the three cap checks |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | A full book must refuse, never drop the order and return `Ok` |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/exchange_cap_test.rs`](tests/exchange_cap_test.rs) | Test Matrix T01 — the cap check, plus Phase P12's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_core/`](../exchange_core/readme.md) — the real caller, via `Exchange::caps_set`/`step_place`
- [`exchange_book/`](../exchange_book/readme.md) — `rests_at`/`level_count`/`account_rests`, the counts the checks take
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p12_cap`, this crate's phase smoke
