# Pitfall: Saturating add hiding an insolvent hold

### Scope

- **Purpose**: Name a specific mistake this crate must avoid.
- **Responsibility**: Using a saturating arithmetic operation where a hold's insufficiency should instead be a hard error.
- **In Scope**: `Holding`'s four edges, and `Escrow::reserve`/`release`/`settle`.

### Statement

A saturating add clamps an overflow to a representable value instead of
signaling that the account cannot actually cover the hold — this turns an
insolvency into a silently-wrong number rather than a refused order, which is
worse than a crash because it looks like success.

### How this crate avoids it

Every balance edit in this crate goes through `checked_add`/`checked_sub`
([`src/lib.rs`](../../src/lib.rs)) — `grep -n "saturating" module/exchange_escrow/src/lib.rs`
returns zero hits. A subtraction that would leave a holding below zero, or an
addition that would leave it past the representable ceiling, returns
`EscrowError::Arithmetic` (or a more specific variant) instead of a clamped
number. Verified directly:
[`tests/reservation_test.rs`](../../tests/reservation_test.rs)'s
`t08_an_unfunded_order_is_rejected_and_nothing_moves` confirms an order the
account cannot cover is refused outright — the balance is never silently
clamped to "as much as fits."

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/015_saturating_add_hiding_insolvent_hold.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:910` | Pitfall in the source's Prompt 7 "Escrow" list |
