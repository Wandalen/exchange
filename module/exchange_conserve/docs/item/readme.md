# item

The exposed surface of `exchange_conserve`, as built.

### Scope

- **Purpose**: What this crate actually exports, and how it compares to the source design's own exposed-item list for it.
- **Responsibility**: One table, every `pub` item, with a short note on fidelity to the proposal.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Per-declaration files.

### Overview Table

| Item | Kind | Signature |
|------|------|-----------|
| `ConserveError` | enum | `{ Notional(TypeError), Overflow, Unbalanced { got: i128 } }` |
| `conserve_assert` | fn | `(&[Trade]) -> Result<(), ConserveError>` |

### Differs from the proposal

The source design (`../../../../../../codename_space_sandbox/intake/core_exchange.txt:610-613`,
catalogued at
[`../../../../docs/exposed_item/014_exchange_conserve_items.md`](../../../../docs/exposed_item/014_exchange_conserve_items.md))
proposes `fill_legs_sum`, `conserve_assert` and
`ConserveError { NotZero, Overflow }`.

- **`fill_legs_sum`** — not built: summing signed legs to zero is
  `exact_arith::money_sum_assert_zero`'s job.
- **`ConserveError`** — `NotZero` is `Unbalanced`, carrying the net; `Notional`
  is added for a trade whose notional is not exact.
- **One signed leg per trade** — built as two, the buyer's and the seller's;
  see the module doc's "Two legs per trade".
