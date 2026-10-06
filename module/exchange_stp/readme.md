# exchange_stp

The self-trade policy, as a closed set of three — a root of the dependency
tree, and this crate does not itself walk the book. Closes feature 15.

```rust
use exchange_stp::{ SelfMatchPolicy, stp_name };

assert_eq!( stp_name( SelfMatchPolicy::CancelIncoming ), "cancel_incoming" );
```

## Wiring

`exchange_match` applies the policy to every candidate pair and re-exports
`SelfMatchPolicy`. The operator picks it per drain:
`exchange_core::Exchange::exchange_step` takes it as an argument.

## Not built: `Allow`

A self-match is always refused, so the source design's `Allow` has nothing to
select. Its other two policies are kept under role names — `CancelResting`
(`CancelOldest`) and `CancelIncoming` (`CancelNewest`) — plus `CancelBoth`,
which the source does not name. See
[`docs/decisions/001_no_allow_resting_incoming_naming.md`](docs/decisions/001_no_allow_resting_incoming_naming.md).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — zero dependencies |
| [`src/lib.rs`](src/lib.rs) | `SelfMatchPolicy` and its stable name |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/decisions/` | Why there's no `Allow`, and the Resting/Incoming naming |
| `docs/pitfall/` | Self-trade allowed by default — why there is no such default here |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| `docs/stp_policy/` | The proposal's own 3 named policy values, mapped to the real enum |
| [`tests/exchange_stp_test.rs`](tests/exchange_stp_test.rs) | Test Matrix T01 — distinctness and naming, plus Phase P04's smoke assertion |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan |

## Related

- [`exchange_match/`](../exchange_match/readme.md) — applies and re-exports `SelfMatchPolicy`; keeps `SelfMatchCancellation`, the match-outcome record
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p04_stp`, this crate's phase smoke
