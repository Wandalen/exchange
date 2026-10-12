# Pitfall: Commit and release of the same hold

### Scope

- **Purpose**: Record the trap named for workstream 002's "Escrow" design area that is this crate's own internal state-machine guarantee, so this crate's own code can show it is actually avoided rather than merely cited.
- **Responsibility**: Document the pitfall, how `exchange_escrow` avoids it, and the code path that verifies the avoidance.
- **In Scope**: `Escrow::release` and `Escrow::settle`'s shared `reservations` ledger — whether a hold can be resolved (committed or released) more than once.
- **Out of Scope**: Whether a *caller* (e.g. `exchange_core`) ever attempts a double-resolution in practice — that is an orchestration question for whichever crate calls `Escrow`, not this crate's own state-machine choice.

### Statement

A hold has exactly one resolution — a fill commits it, anything else releases
it. Applying both to one hold either frees funds already spent or counts
funds already returned twice.

### How this crate avoids it

`release` and `settle` both look the order up in the one `reservations`
ledger before touching an account, and refuse with `NoReservation` once it is
gone. `release` removes the entry; `settle` removes it once nothing is held
and nothing unfilled. Both compute against local copies and commit nothing on
a refusal.

A partial fill leaves the reduced entry in place, and a later `release`
returns what remains — one resolution of the remainder, not a second one of
the filled part.

[`tests/reservation_test.rs`](../../tests/reservation_test.rs)'s
`a_hold_resolves_once` settles then releases, settles twice, releases then
settles, and releases twice; each second attempt is refused and moves
nothing.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../docs/pitfall/014_commit_and_release_of_same_hold.md` | The central catalog's own copy of this pitfall |
| `../../../../../../codename_space_sandbox/intake/core_exchange.txt:909` | Pitfall in the source's Prompt 7 "Escrow" list |
