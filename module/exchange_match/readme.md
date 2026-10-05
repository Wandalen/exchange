# exchange_match

The crossing loop: an incoming order against a book, producing trades, the
quantity it could not get, and any self-match it withdrew instead of
executing.

```rust
use exchange_match::{ cross, SelfMatchPolicy };
// cross( &mut book, &incoming, SelfMatchPolicy::CancelIncoming )
//   -> Crossing { trades, remaining, cancelled }
```

`filled() + remaining == incoming.quantity`, always. The split conserves, and
[`tests/crossing_test.rs`](tests/crossing_test.rs) asserts the sum rather than
only the fill — an engine that filled 4 of 10 and reported a remainder of 0
satisfies a fill-only check and quietly destroys six units of a customer's
order.

## Closes feature 9

`cross` — crossing an incoming order against the opposite side of the book,
producing fills plus a remainder — is feature 9 (`match_in`), the single
entry point into the matching algorithm.

## The row that is easiest to leave out

T07 — an order priced through no liquidity. An engine that fills
unconditionally satisfies the exact-cross and partial-fill cases completely;
only the no-cross case distinguishes a matching engine from a machine that
pairs whatever it is handed. It is asserted here in two forms and again as the
control arm of an external smoke check, whose own top-level assertion is that
the two arms *disagree*.

## Self-match prevention

An incoming order and the resting order it is about to cross may share one
account. The self-match ban this crate enforces is unambiguous that this must
never trade, so the comparison runs inside the loop, before a candidate fill
is built, never after a fill is computed and checked. `SelfMatchPolicy` names which side a caller wants withdrawn —
`CancelResting`, `CancelIncoming`, or `CancelBoth` — and `Crossing.cancelled`
reports exactly what came of it, additively: `remaining` keeps its existing
meaning (`filled() + remaining == incoming.quantity` still holds) rather than
being repurposed to absorb a cancellation.

[`tests/self_match_test.rs`](tests/self_match_test.rs) covers all three
policies and the case a fill-then-check design would get wrong: a self-match
is caught even when the incoming order is larger than the resting quantity, so
no partial trade executes for the part that "fits" before the rest cancels.

## The executed price is the maker's

Decided here, having been left open by the matching algorithm that drives this
crossing loop. The resting order's price is the only price both parties saw
before the trade existed. Executing at the taker's limit instead hands the whole spread to
whichever side happened to arrive second — a fee, charged by the venue, that no
fee schedule accounts for and no participant agreed to.

A taker sweeping several price levels therefore pays each level's own price, not
one blended price. Charging the worst level for all of it, or the best, are both
plausible-looking implementations, and
`each_trade_executes_at_its_own_makers_price` is what separates them.

The rule itself lives in [`exchange_fill`](../exchange_fill/readme.md) as
`Trade::executed_price`, taking both prices and returning one, so that changing
it is an edit in one place with one test on it rather than a search for an
expression inlined in a loop.

## No clock, no hash order, no address

`cross` is a pure function of the book and the incoming order. The arrival-order
invariant this crate must uphold requires that no matching decision reads a
clock, a hash iteration order, or an address; this crate satisfies it by
construction — every input is a value, and the only ordering consulted is the
book's own published sequence.

`crossing_the_same_book_twice_gives_the_same_trades` checks it from inside one
process, which is a weak check: a rule consulting a clock would have to be
unlucky to differ across two adjacent runs. The construction is the guarantee;
the test is the tripwire on it.

## `BookDesynchronized`

`cross` reads the best order, computes a take, and asks the book to consume it.
If the book refuses, the two disagree about what rests — and rather than
clamping or looping, that is an error. It should be unreachable; an error it
cannot silently be wrong about is what makes "should" checkable.

## Time-in-force

`incoming.tif` is read in exactly one place: `tif_requires_full` (from
`exchange_tif`) gates whether `cross` must treat a partial outcome as a
rejection rather than an ordinary partial fill. GTC and IOC need no special
case — `cross` has never rested the incoming order's own remainder for any
TIF (resting it is a caller's decision, not this crate's), so "drop any
remainder" was already the behavior for both; only whether a caller later
rests that remainder differs. FOK is the real branch: by the time `cross`
returns, any trade it made is real and cannot be un-happened, so a FOK order
is probed first against a disposable clone of `book`, and only a probe that
fully consumes it is replayed against the real `book`. An unfillable FOK
comes back shaped exactly like an ordinary no-cross outcome — empty trades,
full remaining, nothing cancelled — because nothing in the real book ever
changed.

[`tests/tif_test.rs`](tests/tif_test.rs) covers IOC's GTC-parity and FOK's
full matrix — fits-entirely, cannot-fill-entirely, against-no-liquidity, and
short-by-a-second-level. The failure mode this probe-then-commit structure
forecloses — a partial fill committed to the real book and then undone — is
this crate's own [`docs/pitfall/001_fok_that_fills_part_then_rejects.md`](docs/pitfall/001_fok_that_fills_part_then_rejects.md).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_book`, `exchange_fill`, `exchange_id`, `exchange_order`, `exchange_side`, `exchange_stp`, `exchange_tif`, `exchange_types`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `cross`, `Crossing`, the crossing predicate, and self-match policy/cancellation types |
| [`tests/crossing_test.rs`](tests/crossing_test.rs) | Test Matrix T05–T07 — full fill, partial fill, and no cross |
| [`tests/tif_test.rs`](tests/tif_test.rs) | Test Matrix — IOC's GTC parity, and FOK's probe-then-commit matrix |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | The 2 "Match policy" pitfalls this crate's FOK probe-then-commit and self-match-before-fill ordering avoid |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/self_match_test.rs`](tests/self_match_test.rs) | Test Matrix T09–T12 — the three self-match policies, checked before a fill commits |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan — the FOK probe-vs-commit gate's one load-bearing condition |

## Related

- [`exchange_book/`](../exchange_book/readme.md) — the book this loop consumes from
- [`exchange_tif/`](../exchange_tif/readme.md) — `tif_requires_full`, consulted here to gate FOK
- [`exchange_stp/`](../exchange_stp/readme.md) — `SelfMatchPolicy`, re-exported here unchanged
