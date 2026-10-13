# exchange_match

The crossing loop: an incoming order against a book, producing trades, the
quantity it could not get, and any self-match it withdrew instead of
executing. Closes feature 9 (`match_in`).

```rust
use exchange_match::{ cross, SelfMatchPolicy };
// cross( &mut book, &incoming, SelfMatchPolicy::CancelIncoming )
//   -> Crossing { trades, remaining, cancelled }
```

`filled() + remaining == incoming.quantity`, always.
[`tests/crossing_test.rs`](tests/crossing_test.rs) asserts the sum, not only
the fill: an engine that filled 4 of 10 and reported a remainder of 0 passes a
fill-only check and destroys six units of a customer's order.

## No cross is an outcome

T07 — an order priced through no liquidity. An engine that fills
unconditionally passes the exact-cross and partial-fill cases; only the
no-cross case tells a matching engine from a machine that pairs whatever it is
handed.

A fill whose price paid cannot be expressed exactly is declined the same way,
before the book gives up the resting order, and the loop stops there rather
than skip to a worse price. A resting remainder no fill can pay then stops
every taker on its side until it is cancelled
(`an_unpayable_fill_at_the_front_stops_the_taker_there`); see the module doc
in [`src/lib.rs`](src/lib.rs).

## Self-match prevention

A pair sharing one account never trades. The comparison runs inside the loop,
before the pair's trade is built. `SelfMatchPolicy` says which side is
withdrawn — `CancelResting`, `CancelIncoming` or `CancelBoth` — and
`Crossing::cancelled` reports it; `remaining` keeps its meaning. Cancel-resting
can withdraw an order and then trade behind it, so each withdrawal records how
many trades preceded it, and `Crossing::steps` gives both in the order they
happened — the order to settle and record them in.

[`tests/self_match_test.rs`](tests/self_match_test.rs) covers the three
policies and the case a fill-then-check design gets wrong: an incoming order
larger than the self-matching quantity trades nothing for the part that
"fits".

## The executed price is the maker's

The resting order's price is the only one both parties saw before the trade.
Executing at the taker's limit hands the spread to whoever arrived second — a
fee nobody agreed to. A taker sweeping several levels pays each level's own
price; `each_trade_executes_at_its_own_makers_price` separates that from the
worst-price and best-price variants. The rule is
`exchange_fill::Trade::executed_price`, one function with one test.

## Time-in-force

`cross` never rests the incoming order, so GTC and IOC differ only in what the
caller does with the remainder. FOK is probed against a clone of the book and
replayed on the real one only if the probe fills it whole; an unfillable FOK
comes back as a no-cross with the book untouched
([`docs/pitfall/001_fok_that_fills_part_then_rejects.md`](docs/pitfall/001_fok_that_fills_part_then_rejects.md)).
A post-only order that `would_take` is refused with
`MatchError::PostOnlyWouldTake`, book untouched.

## Determinism

`cross` reads no clock, hash order or address; the only ordering it consults
is the book's. `crossing_the_same_book_twice_gives_the_same_trades` is a
tripwire on that, not the guarantee — a rule reading a clock would have to be
unlucky to differ across two adjacent runs.

## `BookDesynchronized`

`cross` reads the best order, computes a take and asks the book to consume
it. If the book refuses, the two disagree about what rests; that is an error
rather than a clamp or a retry. It should be unreachable, and an error is what
makes "should" checkable.

## Not built: refusing an inexact tick and lot

An unpayable fill — and the side it can block — needs, through `Exchange`, an
instrument whose tick times lot is inexact at the money scale, such as a
0.0001 tick with a 0.001 lot. An on-grid price is a whole number of ticks and
an on-grid quantity a whole number of lots, so when one tick times one lot is
exact, so is every fill. `exchange_spec::spec_new` does not refuse such a
pair. The refusal is `exchange_spec`'s, and it would refuse the finest grid
`exchange_core`'s tests register on, which they use to reach inexact fills.

## Not built: market orders

An order with no limit price. `exchange_escrow` reserves a buy at its limit,
the most it can be asked to pay; a market buy has none, so nothing bounds what
it reserves. It needs a reservation rule first — a protection price, or the
account's whole available balance — and `Order::price` would become optional
in every crate that reads it.

## Not built: a FOK probe without a clone

`cross` probes a FOK order against a clone of the whole book, every
instrument, on top of the clone `exchange_core::step_place` takes for its dry
run. A read-only walk of one side would do, but it is a second copy of the
loop's rules — price, self-match, settleability — that must agree with
`cross_inner` for good; the replay is safe today because it runs the same
function. Worth it once the dry run stops cloning (`exchange_book`'s readme,
"Not built: an id index").

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_book`, `exchange_conserve`, `exchange_fill`, `exchange_id`, `exchange_order`, `exchange_side`, `exchange_stp`, `exchange_tif`, `exchange_types`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `cross`, `would_take`, `Crossing`, `SelfMatchCancellation`, `MatchError` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | A FOK filling part before it rejects; a self-match checked after the fill |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Exposed surface, as built vs. proposed |
| [`tests/crossing_test.rs`](tests/crossing_test.rs) | Test Matrix T05–T07 — full fill, partial fill, no cross, executed price |
| [`tests/self_match_test.rs`](tests/self_match_test.rs) | Test Matrix T09–T12 — the three self-match policies |
| [`tests/tif_test.rs`](tests/tif_test.rs) | IOC, FOK's probe-then-commit, post-only |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan — the FOK gate's load-bearing condition |

## Related

- [`exchange_book/`](../exchange_book/readme.md) — the book this loop consumes from
- [`exchange_tif/`](../exchange_tif/readme.md) — `tif_requires_full`, `tif_takes`
- [`exchange_stp/`](../exchange_stp/readme.md) — `SelfMatchPolicy`, re-exported here
- [`exchange_conserve/`](../exchange_conserve/readme.md) — checks each batch of trades `cross` returns
