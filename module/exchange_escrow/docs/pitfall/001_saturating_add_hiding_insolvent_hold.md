# Pitfall: Saturating add hiding an insolvent hold

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Using a saturating arithmetic operation where a hold's insufficiency should instead be a hard error.
- **In Scope**: `Holding`'s edges, and `Escrow::reserve`/`release`/`settle`.

### Statement

A saturating add clamps an overflow to a representable value instead of
signalling that the account cannot cover the hold: an insolvency becomes a
wrong number that looks like success.

### How this crate avoids it

Every balance edit goes through `exact_arith`'s checked `+`/`-`, which return a `Result`
(`grep -n "saturating" module/exchange_escrow/src/lib.rs` returns nothing). A
subtraction below zero is refused as `Insufficient` or `NotReserved`, an
addition past the ceiling as `Arithmetic` — never clamped.
[`tests/reservation_test.rs`](../../tests/reservation_test.rs)'s
`t08_an_unfunded_order_is_rejected_and_nothing_moves` and
`a_reservation_stays_releasable_at_the_ceiling` pin both ends.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/015_saturating_add_hiding_insolvent_hold.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:910` | Pitfall in the source's Prompt 7 "Escrow" list |
