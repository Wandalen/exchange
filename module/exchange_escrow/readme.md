# exchange_escrow

The reservation ledger — every account's balance split into `available` and
`reserved`, moved only by `reserve`, `release`, `settle` and the `open`
deposit. Closes hard problem 4 (escrow before rest) and feature 11.

```text
reserve : available -> reserved      ( an order is accepted )
release : reserved  -> available     ( it is cancelled )
settle  : reserved  -> counterparty  ( it fills; price improvement back to available )
open    : outside   -> available     ( a deposit )
```

`reserved` is *stored*, not derived: one recomputed by summing resting orders
agrees with the book by construction and can never catch a disagreement with
it. Stored, the two are independent, and
`the_stored_reserve_agrees_with_the_orders_behind_it` compares them.

## Reserve before the book, whole or not at all

An order that rests before its funds are held can be filled against money
nobody has. A partial reservation leaves it resting for a quantity its owner
cannot cover, found short by whoever crosses it. So `reserve` runs before the
order reaches the book, and refuses the order whole.

## The ledger, not the caller, says whose and at what limit

Each reservation records its order's account and limit. `release` and
`settle` refuse any account but the one that reserved
(`EscrowError::NotOwner`), and `settle` takes the buyer's limit from its
reservation, refusing a trade priced above it (`NotReserved`).

## Price improvement, released

A buyer reserves at its own limit and may execute better. Paying 2.00 against
a 2.50 reservation leaves 0.50 per unit that belongs to the buyer, and
`settle` releases it in the same operation as the transfer. Without this the
spread piles up in `reserved` while every total still balances —
`a_price_improvement_returns_the_unused_reservation` pins it.

## `BTreeMap`, not a hash map

Iteration order is part of the output whenever a total is summed or a report
is produced, and a hash map's order is a function of its seed. A `BTreeMap`
has no such order to leak, at a logarithmic lookup nothing here has measured
as a constraint.

## `Conserved`

The edges are the same for cash and for the asset; the two differ only in
what "cannot go below zero" means. `Money` is signed and guarded explicitly,
`Quantity` refuses a negative itself. So `Holding<T>` is generic over a
`Conserved` trait — `NOTHING`, `checked_plus`, `checked_minus` — and the
rules are written once.

Three guards, each on a different value. `checked_minus` keeps a *balance*
from going below zero. `Holding::movement` refuses a negative *amount* before
any edge runs — without it a negative price ran every edge backwards, and no
balance went below zero to notice. `receive` keeps a holding's *total* within
the ceiling, so a reservation always has room to come back.

`Holding`'s edges are private. Value moves only through `Escrow`'s own
methods, so nothing changes one side of the partition without the other.

## Not built: per-asset holdings

`Account` holds one cash and one asset `Holding`, whatever the instrument, so
two instruments on one exchange draw on the same holdings — hard problem 19
stays open. Keying holdings by `AssetId` needs each order's base and quote
asset at `reserve` and each trade's at `settle`. `Order` carries only its instrument,
the instrument's assets live in `exchange_core`'s specs, and `Trade` carries
no instrument at all (`exchange_fill`'s readme). Until one of them does, an
account holds one cash and one asset.

## Not built: withdrawal

`open` deposits; nothing takes funds out. Balances are workstream 010's by the
source design
([`../../docs/decision/003_010_implements_escrow_and_consumes_fills.md`](../../docs/decision/003_010_implements_escrow_and_consumes_fills.md)),
and a withdrawal edge here would widen the boundary breach that decision
records rather than close it.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exchange_fill`, `exchange_id`, `exchange_order`, `exchange_side`, `exchange_types`, `exact_arith` |
| [`src/lib.rs`](src/lib.rs) | `Escrow`, `Account`, `Holding`, `Conserved`, and settlement |
| `docs/definition/` | Module Index — every `pub` item against where it's documented |
| `docs/item/` | Exposed surface, as built vs. the proposal's `EscrowPort` |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/pitfall/` | A saturating add hiding an insolvent hold; one hold resolved twice |
| [`tests/reservation_test.rs`](tests/reservation_test.rs) | Test Matrix T08–T10 — coverage, release, and the conservation totals |

## Related

- [`exchange_types/`](../exchange_types/readme.md) — the obligations this ledger holds
- [`exchange_core/`](../exchange_core/readme.md) — reaches this crate through its `EscrowPort` trait
- [`smoke_exchange_phases/`](../smoke_exchange_phases/readme.md) — `demo_p14_hold`, `demo_p15_nohold`
- [`exact_arith/`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md) — `verify`, which audits the postings `exchange_core` builds from the trades
