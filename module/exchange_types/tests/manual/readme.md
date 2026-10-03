# exchange_types — manual testing plan

The automated suite pins `notional()`'s exact-or-refused arithmetic,
`Trade::executed_price`'s maker-price rule, and the order/obligation shapes
byte for byte. This plan covers what no assertion in it reaches: whether the
guards behind those pins are load-bearing rather than redundant, and whether
a type this crate defines can be extended later without silently breaking a
consumer that was never updated to notice.

Every check below is a command with an expected observation. Run them from the
workspace root. Checks that edit a source file say so, and each ends by
reverting.

## M1 — the inexact-notional guard is load-bearing

`notional()` refuses rather than rounds when a price × quantity product does
not land exactly on the currency's own scale. A caller "simplifying" this
crate could reasonably replace the refusal with silent truncation — the guard
only earns its place if removing it actually breaks something a real caller
depends on, rather than only ever being exercised by its own test.

```bash
# In substrate/exchange/exchange_types/src/lib.rs, in notional(),
# replace the condition  if product % scale != 0  with  if false
cargo nextest run -p exchange_types --all-features 2>&1 | grep -E 'FAIL|Summary'
```

**Expected:** `a_notional_needing_rounding_is_refused` fails, and it fails by
returning `Ok` with a silently-truncated amount rather than by returning the
wrong error — the guard's absence doesn't get caught downstream, it gets
laundered into a wrong number.

**Observed 2026-09-13:** exactly that —

```
thread 'a_notional_needing_rounding_is_refused' panicked at .../order_types_test.rs:104:3:
assertion `left == right` failed
  left: Ok(Decimal { minor: 0 })
 right: Err(NotionalInexact)
```

One minor unit of quantity at one minor unit of price (`0.000001 × 0.000001`,
twelve decimal places against the currency's six) truncated to `0` rather than
refusing — a settlement of nothing, silently accepted. Reverted; a second run
confirmed 11/11 tests pass again.

## M2 — a trade executes at the maker's price, not the taker's

`Trade::executed_price` is asserted equal to the maker's price already, but an
assertion that a function returns argument A is only as strong as the test's
own ability to tell A from B apart. If the two prices happened to be equal,
the same test would pass whichever operand the function actually returned.

```bash
# In substrate/exchange/exchange_types/src/lib.rs, in
# Trade::executed_price, swap which argument is ignored:
#   pub const fn executed_price( maker : Price, _taker : Price ) -> Price { maker }
#   ->
#   pub const fn executed_price( _maker : Price, taker : Price ) -> Price { taker }
cargo nextest run -p exchange_types --all-features 2>&1 | grep -E 'FAIL|Summary'
```

**Expected:** `a_trade_executes_at_the_makers_price` fails, with the taker's
price reported where the maker's was expected.

**Observed 2026-09-13:** exactly that —

```
thread 'a_trade_executes_at_the_makers_price' panicked at .../order_types_test.rs:171:3:
assertion `left == right` failed
  left: Decimal { minor: 3000000 }
 right: Decimal { minor: 2500000 }
```

A generous taker bidding `3.00` against a maker resting at `2.50` executed at
`3.00` under the mutation — the taker's own price, exactly the
give-away-value-for-free bug the maker's-price rule exists to prevent.
Reverted; a second run confirmed 11/11 tests pass again.

## M3 — nothing in the family can silently absorb a new `EventKind` variant

`EventKind` is this crate's own type, and every consumer of it lives in a
sibling crate this file can't pin a test into. The structural risk a manual
check has to cover instead: a `match`/`matches!` on `EventKind` with a
wildcard or catch-all arm would keep compiling — and keep passing — the day a
new variant is added here, silently treating it as whatever the wildcard arm
does rather than failing to compile until every site is updated.

```bash
grep -rn 'EventKind::' substrate/exchange/*/src/*.rs | grep -v '_test\.rs'
```

**Expected:** every non-test usage of `EventKind` names a specific variant —
none is reached through a bare `_ =>` or an unnamed-binding wildcard arm.

**Observed 2026-09-13:** the only substantive usage outside this crate is in
`exchange_core::postings`/`exchange_core::parties`
(`exchange_core/src/lib.rs:450` and `:478`):

```rust
let EventKind::Trade( trade ) = event.kind else { continue; };
...
event.order == trade.taker && matches!( event.kind, EventKind::OrderAccepted { side : Side::Buy, .. } )
```

Both name one variant exactly (`Trade`, `OrderAccepted`); the `..` in the
second elides that variant's own fields, not other variants. A new
`EventKind` variant added today would be invisible to both sites rather than
silently mishandled by them — the family has no wildcard for it to fall into.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-09-13 | M1–M3 | All three ran. M1 and M2 each disproved-by-mutation that their guard/assertion was redundant — the notional guard laundered dust into a silent zero rather than refusing it, and the maker's-price assertion would have passed a taker-price implementation just as easily before the mutation revealed which operand the test actually pins. M3 confirmed no `EventKind` consumer in the family holds a wildcard arm a future variant could silently fall into. |
