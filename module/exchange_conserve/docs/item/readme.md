# item

The exposed surface of `exchange_conserve`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice; see this
family's own `docs/research/` for why a fuller `item_des.rulebook.md`
per-declaration split is deferred).

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a short note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above).

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `ConserveError` | enum | `{ Notional(TypeError), Overflow, Unbalanced }` |
| `fill_legs_sum` | fn | `(&[Money]) -> Result<Money, ConserveError>` |
| `conserve_assert` | fn | `(&[Trade]) -> Result<(), ConserveError>` |

### Differs from the proposal

The source design's own exposed-item list
(`../../../../../../codename_space_sandbox/intake/core_exchange.txt:610-613`,
also catalogued centrally at
[`../../../../docs/exposed_item/014_exchange_conserve_items.md`](../../../../docs/exposed_item/014_exchange_conserve_items.md) —
whose own "Folded into `exchange_escrow`" Design status is now stale, see
below) proposes `fill_legs_sum`, `conserve_assert`, and
`ConserveError{NotZero,Overflow}`. The real build matches the two function
names exactly; `ConserveError` has 3 variants rather than 2 — `Notional`
(a trade's own price/quantity isn't expressible) and `Unbalanced` (the
proposal's `NotZero`, renamed) are both present, plus `Overflow`. Beyond
naming, this crate's own module doc records a real design revision: the
first implementation (one signed leg per trade) could never be called
correctly from `exchange_match::cross` — every trade one call produces
shares the same `taker_side`, so a same-signed batch could never sum to
zero — and was replaced with the shipped **both-legs-per-trade** design,
making `ConserveError::Unbalanced` structurally unreachable today (kept
deliberately, for the day a fee enters the design and a leg stops
canceling automatically).

### Not extracted from `Escrow::total_cash`/`total_asset`

The source design names those two `Escrow` methods as this crate's extraction
source — they check the *ledger's* own internal self-consistency (trivially
true by construction), a different check at a different grain from this
crate's actual job: whether *one batch of fills*, on its own with no ledger
in sight, nets to zero. Full reasoning: this crate's own module doc
([`../../src/lib.rs`](../../src/lib.rs), "Not extracted from
`Escrow::total_cash`/`total_asset`") and [`../../readme.md`](../../readme.md)
— not restated here to keep one home for it.

### Central doc staleness found while redistributing

Both central `docs/crate/014_exchange_conserve.md` and
`docs/exposed_item/014_exchange_conserve_items.md` said "Folded into the
real `exchange_escrow` crate... no standalone `conserve_assert` function
exists" — stale: this crate is real, built, and tested independently of
`exchange_escrow`. Thinned to pointers as part of this redistribution pass.
