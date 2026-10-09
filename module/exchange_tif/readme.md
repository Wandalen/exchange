# exchange_tif

Time-in-force as an explicit value on the order: GTC, IOC, FOK, post-only — a
root of the dependency tree, and this crate does not itself match. Closes hard
problem 20 and feature 16.

```rust
use exchange_tif::{ Tif, tif_requires_full, tif_rests, tif_takes };

assert!( tif_rests( Tif::Gtc ) );
assert!( !tif_rests( Tif::Ioc ) );
assert!( tif_requires_full( Tif::Fok ) );
assert!( !tif_takes( Tif::PostOnly ) );
```

## Wiring

All four values are reachable end to end: `InboundCmd::Place` carries the
order's own `tif` into `exchange_core::Exchange::exchange_step`.

| Function | Consulted by | Effect |
|----------|--------------|--------|
| `tif_requires_full` | `exchange_match::cross` | A FOK that cannot fill entirely is rejected before any trade |
| `tif_rests` | `exchange_core` (`step_place`), `exchange_inbound` (`inbound_apply`) | An IOC/FOK remainder is dropped instead of resting |
| `tif_takes` | `exchange_match::cross`, `exchange_core` (`step_place`) | A post-only order that would take is refused whole |

## Post-only

Not in the source design. A post-only order rests like GTC but never takes;
it is a `Tif` value rather than a flag on `Order` — see
[`docs/decisions/001_post_only_is_a_tif.md`](docs/decisions/001_post_only_is_a_tif.md).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `Tif` and its three query predicates |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why post-only is a `Tif` value |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| `docs/tif/` | The proposal's own 3 named time-in-force values |
| [`tests/exchange_tif_test.rs`](tests/exchange_tif_test.rs) | Test Matrix T01 — the four dispositions, plus Phase P03's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_match/`](../exchange_match/readme.md) — gates FOK on `tif_requires_full`, refuses a taking post-only on `tif_takes`
- [`exchange_core/`](../exchange_core/readme.md) — drops a non-resting remainder on `tif_rests`
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p03_tif` and `demo_p22_ioc`, this crate's phase smokes
