# exchange_types

The exact-settlement logic every other crate shares — `Price`, `TypeError`,
`notional`, `obligation` — and nothing else. Every vocabulary type this
crate used to re-export now has its own dedicated crate, and every former
consumer imports from that crate directly (see Extraction, below). Nothing
here acts on `Trade`/`Event` directly any more; that moved to
`exchange_fill`.

```rust
use exchange_types::notional;
use exact_arith::{ Money, Quantity };

let price = Money::parse( "2.50" ).unwrap();
let quantity = Quantity::from_int( 4 ).unwrap();
assert_eq!( notional( price, quantity ).unwrap(), Money::parse( "10" ).unwrap() );
```

## Two prohibitions, and where they come from

**No ECS type.** Not `Entity`, not `World`, not `Component`, not `Query`, not a
system registration — anywhere in the family, not merely in its public API.
The failure the rule forbids is a private field: an exchange holding an `Entity`
handle to identify a trader compiles, exposes nothing, passes every behavioural
test, and has coupled the market to the simulation's storage layer exactly as
completely as a public one would. So the check is a source walk rather than a
signature review — an external contract test reads every `.rs` file under all
five crates' `src/`, strips comments, and fails on a whole word or on a
`CamelCase` identifier derived from one. `EntityId` is exactly the shape an
ECS handle takes when it arrives under a convenience alias, and a plain
`grep -w` lets it through.

**No float.** Every price, quantity and balance is
[`exact_arith`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md), and the same walk fails on
`f32` or `f64` in code. The two lines in this crate's own module docs that name
`Entity` and `World` are the only mentions of either word in the family, and
they are the sentence stating the ban.

## One decision this crate still makes

**Notional is exact or it is an error** (below) — the one piece of real
logic still declared here. The naming/shape decisions about `Trade` itself
(`Trade`-not-`Fill`, both sides named in the record, `executed_price` as a
named function) moved with the type to `exchange_fill` — see that crate's
own [`src/lib.rs`](../exchange_fill/src/lib.rs) module doc for the reasoning;
not repeated here since this crate no longer declares `Trade`.

## Notional is exact or it is an error

`price × quantity` at scale 6 each lands at scale 12 and has to come back to 6.
The division is exact-or-refuse: a remainder is `NotionalInexact`, never a
rounding. Rounding a settlement amount is how value leaks one minor unit at a
time, in the direction whoever chose the rounding mode preferred, and the leak
balances perfectly in every per-account sum.

The multiply happens in `i128` and the result is narrowed back to `Backing`,
so an intermediate that exceeds the width is `NotionalOutOfRange` rather than a
wrap.

## Extraction — retired as a re-export aggregator

`Side`, `AccountId`, `OrderId`, `Sequence` moved out in Stage 1; `Order` and
`Obligation` moved out next, to `exchange_order`, gaining the `instrument`/
`tif` fields the real struct was missing. `Trade`, `Event`, `EventKind`,
`RejectReason` and `CancelCause` moved out next again, to `exchange_fill`,
gaining a `taker_side` field on `Trade`. All eleven were re-exported here
unchanged for a time, so every existing `use exchange_types::{ ... }` kept
resolving through the move — but every call site has since been cut over to
the leaf crate directly, and the re-export block is gone. This crate's own
`Amount` alias was dropped the same way: the real `Amount` lives in
`exchange_order`, so a caller that needs it now reaches it there.
`notional`/`TypeError`/`obligation` stay — this crate depends forward on
`exchange_order` and `exchange_side` for the types `obligation`'s own
signature and body name, as private imports rather than public re-exports,
rather than the reverse. The crate itself is not retiring — this narrowed
Price/TypeError/notional/obligation role is its settled, permanent shape.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| [`Cargo.toml`](Cargo.toml) | Manifest — `exact_arith`, `exchange_order`, `exchange_side`; dev-only `exchange_id`, `exchange_tif` |
| [`src/lib.rs`](src/lib.rs) | `Price`, `TypeError`, `notional`, and the obligation rule — nothing re-exported |
| `docs/workaround/` | External constraints this crate absorbs — none |
| `docs/definition/` | Module index — every `pub` item and where it's documented |
| `docs/item/` | Consolidated exposed-surface listing, as built vs. proposed |
| [`tests/order_types_test.rs`](tests/order_types_test.rs) | Test Matrix T01 — `obligation`/`notional` exactness |
| [`tests/manual/readme.md`](tests/manual/readme.md) | Manual plan — guard load-bearing checks, `EventKind` wildcard sweep |

## Related

- [`exact_arith/`](https://github.com/Wandalen/exact/blob/master/module/exact_arith/readme.md) — every price and balance in this crate
- [`exchange_order/`](../exchange_order/readme.md) — `Order`/`Obligation`/`Amount` now live there; this crate depends forward on it for the two names `obligation`'s own signature needs
