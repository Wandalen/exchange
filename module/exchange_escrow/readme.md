# exchange_escrow

The reservation ledger — every account's balance split into `available` and
`reserved`, with four edges between them and no fifth.

```text
reserve : available -> reserved
release : reserved  -> available
deliver : reserved  -> gone      ( settlement, outgoing )
receive : gone      -> available ( settlement, incoming )
```

The available/reserved separation invariant this crate must uphold requires the
pair to be *stored*, not derived: a `reserved` recomputed on demand by summing
resting orders is a number that agrees with the book by construction and can
therefore never detect a disagreement with it. Stored, the two are
independent, and `the_bridge_assertion_holds_across_a_partial_fill` can compare
them.

## Reserve before the book, whole or not at all

The escrow-covers-resting-orders invariant this crate must uphold:
the reservation precedes book insertion. An order that rests before its funds
are held is an order that can be filled against money nobody has, and the
window is not narrow in a system that intends to be replayable — it is every
order, every time, waiting for the failure that finds it.

Reserve-or-reject-whole, too. A partial reservation leaves an order resting for
a quantity its owner cannot cover, and the shortfall is discovered by whoever
crosses it.

## Price improvement, released

A buyer reserves at its own limit and may execute better. Paying 2.00 against a
2.50 reservation leaves 0.50 per unit that belongs to the buyer, and `settle`
releases it in the same operation as the transfer.

Without this the exchange accumulates the spread in `reserved` — silently, while
every per-account conservation sum still balances perfectly, because the money
is still in the right account, merely unreachable. `price_improvement_is_released_to_the_buyer`
pins it.

## `BTreeMap`, not a hash map

Iteration order is part of the output whenever a total is summed or a report is
produced, and a hash map's order is a function of the hasher's seed. The
arrival-order invariant this crate must uphold forbids a matching decision
from reading a hash iteration order; storing the accounts in a `BTreeMap`
means there is no such order to read. It costs a logarithmic lookup, which
nothing here has measured as a constraint.

## `Conserved`

The four edges are the same for cash and for asset, and the two differ only in
what "cannot go below zero" means for their type — `Money` is signed and is
guarded explicitly, `Quantity` refuses it in its own constructor. So `Holding<T>`
is generic over a `Conserved` trait with `NOTHING`, `checked_plus` and
`checked_minus`, and the movement rules are written once.

There are two such guards, not one, and the difference is which value they
constrain. `Conserved::checked_minus` constrains the *balance* after a
subtraction: no holding ends below zero. `Holding::movement` constrains the
*amount* before any edge runs: a negative amount is refused outright with
`EscrowError::NegativeAmount`. The second was missing until a negative price
made a buy's notional negative, at which point every edge ran backwards —
`reserve` grew `available` instead of shrinking it, and the balance guard saw
nothing wrong because the balance genuinely had not gone below zero. Both
guards are needed; neither implies the other.

`Holding`'s four edges are private to this crate. The only way to move value is
`reserve`, `release` and `settle` on `Escrow`, so there is no path that changes
one side of the partition without the other.

## One asset per account

`Account` holds one cash and one asset `Holding`, whatever the instrument. The
`base`/`quote` an `InstrumentSpec` names are not read here, so two instruments
on one exchange draw on the same holdings — hard problem 19 stays open until
holdings are keyed by `AssetId`.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_fill`, `exchange_id`, `exchange_order`, `exchange_side`, `exchange_types`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `Escrow`, `Account`, `Holding`, `Conserved`, and settlement |
| `docs/definition/` | Module Index — every `pub` item against where it's documented |
| `docs/item/` | The exposed surface, as built, against the proposal's own `EscrowPort` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | The 2 "Escrow" pitfalls that are purely this crate's own arithmetic/state-machine choices |
| [`tests/reservation_test.rs`](tests/reservation_test.rs) | Test Matrix T08–T10 — coverage, release, and the conservation totals |

## Related

- [`exchange_types/`](../exchange_types/readme.md) — the obligations this ledger holds
- [`exact_arith/`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md) — `verify`, which audits the postings this produces
