# smoke_exchange_core

One order crossing five crates, watched from outside.

```bash
cargo run -p smoke_exchange_core
```

```text
  crossing arm : 1 trade(s); seller 1010, buyer 990
  control  arm : 0 trade(s)
  cancel   arm : 1000 available again
smoke_exchange_core: ok
```

## Three arms, and why the second one exists

**Crossing.** A bid meets a resting ask, one trade results, the seller ends with
1010 and the buyer with 990, and the postings the exchange emits sum to zero
under the exchange's own `verify`.

**Control.** A bid one minor unit below the ask. Zero trades, asserted — both
orders rest. Without this arm the lane proves only that the engine *can* fill,
which a machine that pairs whatever it is handed also does. `main` asserts the
two arms disagree, so the lane cannot pass by filling everything or by filling
nothing.

**Cancel.** An account with a resting order shows 975 available and 25 reserved;
after the cancel it shows 1000 and 0. Release is event-atomic — there is no
observable moment where the order is gone and the money is still held.

## One dependency, on purpose

The lane depends on [`exchange_core`](../exchange_core/readme.md) and nothing
else — not on any of the crates behind it. A re-export missing from the
facade is therefore a build failure here rather than a gap nobody notices
until the first external consumer arrives.

## Grading

The lane's exit code is the verdict, and is this crate's own reached-test.
`assert!` is the mechanism; a panic is a failed lane.

`tests/lane_test.rs` does not replace that, it runs it. A lane that is its own
test still needs something to run it, and while the arms lived in `src/main.rs`
the only thing that ever did was a person typing `cargo run` — and no test
suite can reach a bare `src/main.rs` to verify it automatically. So the arms
moved into `src/lib.rs` and the suite now drives
them, which changes nothing about what the lane asserts and everything about how
often anyone finds out.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`docs/`](docs/readme.md) | The external constraint this crate absorbs — none, verified against its dependency graph and source |
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_core` alone |
| [`src/lib.rs`](src/lib.rs) | The three arms and the disagreement assertion |
| [`src/main.rs`](src/main.rs) | The lane's process entry point, and nothing else |
| [`tests/lane_test.rs`](tests/lane_test.rs) | Runs the lane, and each arm on its own, from the suite |

## Related

- [`exchange_core/`](../exchange_core/readme.md) — the facade this lane grades
