# exchange_snap — manual testing plan

The automated suite pins two properties the type signatures alone can't
guarantee: that each row carries *remaining*, not *submitted*, quantity, and
that row order matches the book's own priority order rather than insertion
or hash order. This plan checks whether those two assertions are actually
load-bearing rather than redundant with something else in the suite.

## M1 — rows actually carry remaining, not submitted, quantity

```bash
# In substrate/exchange/module/exchange_snap/src/lib.rs, in snap_take's
# closure, replace  qty : resting.remaining  with  qty : resting.order.quantity
cargo test -p exchange_snap --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `snap_take_copies_every_resting_order_with_its_remaining_quantity`
fails — a partially-filled order's snapshot would report its original size.

**Observed 2026-10-03:** exactly that —

```
thread 'snap_take_copies_every_resting_order_with_its_remaining_quantity' panicked:
assertion `left == right` failed
  left: [RestRow { order: OrderId(1), ..., qty: Qty { value: Decimal { minor: 5000000 } } }]
 right: [RestRow { order: OrderId(1), ..., qty: Qty { value: Decimal { minor: 3000000 } } }]

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; see Run Record.

## M2 — row order actually matches the book's priority order

```bash
# In snap_take, after `.collect()`, insert a line reversing the Vec:
#   let mut rows : Vec< RestRow > = book.iter()....collect();
#   rows.reverse();
cargo test -p exchange_snap --all-features 2>&1 | grep -E 'FAILED|test result'
```

**Expected:** `rows_preserve_the_books_priority_order` fails — rows would
come back asks-then-bids, each side worst-first.

**Observed 2026-10-03:** exactly that —

```
thread 'rows_preserve_the_books_priority_order' panicked:
assertion `left == right` failed: best bid first, then best ask first — exchange_book's own published order
  left: [3, 4, 1, 2]
 right: [2, 1, 4, 3]

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Reverted; a final run confirmed 5/5 tests pass again, and `cargo clippy
--all-targets --all-features -- -D warnings` stayed clean.

## Run Record

| Date | Checks | Result |
|------|--------|--------|
| 2026-10-03 | M1, M2 | Disproved-by-mutation that both assertions are redundant — disabling either one is caught immediately, by exactly the test expected to catch it and no others. |
