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

The rule itself lives in [`exchange_types`](../exchange_types/readme.md) as
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

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_book`, `exchange_types`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `cross`, `Crossing`, the crossing predicate, and self-match policy/cancellation types |
| [`tests/crossing_test.rs`](tests/crossing_test.rs) | Test Matrix T05–T07 — full fill, partial fill, and no cross |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | The 1 "Match policy" pitfall this crate's FOK probe-then-commit avoids |
| [`tests/self_match_test.rs`](tests/self_match_test.rs) | Test Matrix T09–T12 — the three self-match policies, checked before a fill commits |

## Related

- [`exchange_book/`](../exchange_book/readme.md) — the book this loop consumes from
