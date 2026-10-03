# Pitfall: Commit and release of the same hold

### Scope

- **Purpose**: Record the trap named for workstream 002's "Escrow" design area that is this crate's own internal state-machine guarantee, so this crate's own code can show it is actually avoided rather than merely cited.
- **Responsibility**: Document the pitfall, how `exchange_escrow` avoids it, and the code path that verifies the avoidance.
- **In Scope**: `Escrow::release` and `Escrow::settle`'s shared `reservations` ledger — whether a hold can be resolved (committed or released) more than once.
- **Out of Scope**: Whether a *caller* (e.g. `exchange_core`) ever attempts a double-resolution in practice — that is an orchestration question for whichever crate calls `Escrow`, not this crate's own state-machine choice.

### Statement

A hold has exactly one valid resolution — a fill commits it, anything else
releases it — and applying both operations to the same hold either
double-frees funds that were already spent or double-counts funds that were
already returned.

### How this crate avoids it

Both resolution paths consult the same `self.reservations: BTreeMap<OrderId, Obligation>`
before mutating anything, and both remove (or shrink) the order's entry as
part of resolving it — there is exactly one ledger, not one counter per
operation ([`src/lib.rs`](../../src/lib.rs)):

- `Escrow::release` looks up `self.reservations.get(&order)` first and
  returns `Err(EscrowError::NoReservation(order))` if nothing is there —
  exactly the state left behind by an earlier `release` (which calls
  `self.reservations.remove(&order)`) or an earlier fully-discharging
  `settle` (whose `commit_reservation` also removes the entry once nothing
  remains, via `reduced_obligation` returning `None`).
- `Escrow::settle`'s `reduced_obligation` helper performs the identical
  lookup (`current.ok_or(EscrowError::NoReservation(order))?`) against the
  same map, for both the taker's and the maker's reservation, before either
  account is touched. A hold already resolved by an earlier `release` is
  therefore refused by a later `settle` the same way, and vice versa.
- Both functions compute every fallible step against local copies and
  commit nothing until every step has succeeded (`settle`'s own doc
  comment: "every step below runs against local copies... before anything
  is written back into `self`"), so a refused double-resolution changes
  nothing rather than partially applying one side of the operation.

A partial fill is not a counterexample: `reduced_obligation` can leave
`Some(reduced amount)` in the map rather than removing it, and a later
`release` of that same order then correctly releases whatever remains — one
resolution of the *whole* remaining hold, not a second resolution of the
already-settled portion.

Not yet backed by a test narrating this exact double-resolution scenario by
name — the closest existing coverage is
[`tests/reservation_test.rs`](../../tests/reservation_test.rs)'s
`releasing_without_a_reservation_is_refused`, which exercises the same
`self.reservations.get` → `NoReservation` code path that a post-settle or
post-release double-resolution would also hit, just not phrased as "this
order was already resolved once." Documented honestly here rather than
claiming dedicated coverage that does not exist.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/014_commit_and_release_of_same_hold.md` | The central catalog's own (now-relocated) copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:909` | Pitfall in the source's Prompt 7 "Escrow" list |
