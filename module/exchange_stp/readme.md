# exchange_stp

The self-trade policy, as a closed set of three — a root of the dependency
tree, and this crate does not itself walk the book.

```rust
use exchange_stp::{ SelfMatchPolicy, stp_name };

assert_eq!( stp_name( SelfMatchPolicy::CancelIncoming ), "cancel_incoming" );
```

## Closes feature 15

`SelfMatchPolicy` offering a closed choice of self-trade policies,
configured once per book rather than decided ad hoc at match time, is
feature 15 (`stp_policy`) — `exchange_match` is the crate that applies it on
every candidate pair; see that crate's own readme.

## Extracted from `exchange_match`

`SelfMatchPolicy` lived in `exchange_match` until this crate split out;
`exchange_match` now depends on this crate and re-exports it, so every
existing `use exchange_match::SelfMatchPolicy` (and the `exchange_core`
re-export chain above it) still resolves.

## Not built: `Allow`

The source design names three policies, one of which (`Allow`) lets a
self-match cross like any other pair. The real design never had one — see
[`docs/decisions/001_no_allow_resting_incoming_naming.md`](docs/decisions/001_no_allow_resting_incoming_naming.md)
for the full reasoning. This crate keeps the real three: `CancelResting`,
`CancelIncoming`, `CancelBoth`, functionally equivalent to the source's
`CancelOldest`/`CancelNewest` plus one the source doesn't name.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `SelfMatchPolicy` and its stable name |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why there's no `Allow`, and the Resting/Incoming naming |
| `docs/pitfall/` | The 1 "Match policy" pitfall this crate's variant set bears on |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| `docs/stp_policy/` | The proposal's own 3 named policy values, mapped to the real enum |
| [`tests/exchange_stp_test.rs`](tests/exchange_stp_test.rs) | Test Matrix T01 — distinctness and naming, plus Phase P04's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_match/`](../exchange_match/readme.md) — re-exports `SelfMatchPolicy`, and keeps `SelfMatchCancellation` (the match-outcome record, not pure policy vocabulary)
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p04_stp`, this crate's phase smoke
