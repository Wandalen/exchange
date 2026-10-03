# exchange_tif

Time-in-force as an explicit value on the order: GTC, IOC, FOK — a root of the
dependency tree, and this crate does not itself match.

```rust
use exchange_tif::{ Tif, tif_requires_full, tif_rests };

assert!( tif_rests( Tif::Gtc ) );
assert!( !tif_rests( Tif::Ioc ) );
assert!( tif_requires_full( Tif::Fok ) );
```

## Genuinely new

No time-in-force concept existed anywhere in the real crates before this one
— `exchange_types`' own module documentation named the gap directly. This
crate closes it: hard problem 20, feature 16.

Wiring, current as of this writing (superseded note: this section
previously said "nothing yet calls `tif_rests`/`tif_requires_full` from the
matching loop" — stale, both calls exist now):
[`tif_requires_full`] is consulted by `exchange_match::cross` to gate FOK,
with a real regression suite
(`exchange_match/tests/tif_test.rs`). [`exchange_core::Order`] now carries a
real `tif` field. The one remaining gap is `exchange_core::Exchange::submit`
itself, which still hardcodes every constructed order to `Tif::Gtc` rather
than taking a caller-supplied value — so neither IOC nor FOK is reachable
end-to-end through the real facade yet, even though `exchange_match`'s own
FOK gate is real and tested. [`tif_rests`] (IOC's remainder-discard) is not
yet consulted by `cross`/`submit` at all — it is exercised only by
[`smoke_exchange_phases`]'s own `demo_p22_ioc` orchestration, outside the
real facade.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `Tif` and its two query predicates |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| `docs/tif/` | The proposal's own 3 named time-in-force values |
| [`tests/exchange_tif_test.rs`](tests/exchange_tif_test.rs) | Test Matrix T01 — the three dispositions, plus Phase P03's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p03_tif` and `demo_p22_ioc`, this crate's phase smokes
- [`exchange_match/`](../exchange_match/readme.md) — consults `tif_requires_full` in `cross` to gate FOK
