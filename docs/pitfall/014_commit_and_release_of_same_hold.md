# Pitfall: Commit and release of the same hold

**Relocated.** This pitfall is single-crate-relevant (`exchange_escrow`'s own
state-machine guarantee over its `reservations` ledger — verified enforced by
both `release` and `settle` consulting the same map and refusing a
double-resolution with `EscrowError::NoReservation`) and now lives at
[`../../module/exchange_escrow/docs/pitfall/002_commit_and_release_of_same_hold.md`](../../module/exchange_escrow/docs/pitfall/002_commit_and_release_of_same_hold.md),
with the "how this crate avoids it" verification alongside it. The other
three "Escrow" pitfalls (011–013) stay here — each describes a call-sequence
invariant spanning this crate and whichever crate calls it, not a choice
internal to this crate alone.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/intake/core_exchange.txt:909` | Pitfall in the source's Prompt 7 "Escrow" list |
