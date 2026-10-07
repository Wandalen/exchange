# item

The exposed surface of `exchange_types`, as built — a consolidated index
rather than one file per declaration (the lighter-pass choice; see this
family's own `docs/research/` for why a fuller `item_des.rulebook.md`
per-declaration split is deferred).

### Scope

- **Purpose**: What this crate actually exports — now just the logic it declares itself, since its former re-export surface was retired.
- **Responsibility**: One table, every exposed item, declared here.
- **In Scope**: This crate's own public surface.
- **Out of Scope**: Full `item_des.rulebook.md`-style per-declaration files (deferred, see above); the 11 types this crate used to re-export — each now belongs solely to its real owning crate's own `docs/item/`, with no row here at all.

### Overview Table

| Item | Kind | Signature | Owner |
|------|------|-----------|-------|
| `TypeError` | enum | `{ NotionalOutOfRange, NotionalInexact }` | here |
| `notional` | fn | `(Price, Quantity) -> Result<Money, TypeError>` | here |
| `obligation` | fn | `(&Order) -> Result<Obligation, TypeError>` | here |

`Order`/`Obligation` (from `exchange_order`) and `Side` (from `exchange_side`)
are named in `obligation`'s own signature and body, so those two crates stay
real dependencies — but as private `use` imports, not `pub use` re-exports,
so neither appears as a row here any more. The 11 types this crate once
re-exported (`Trade`, `Event`, `EventKind`, `RejectReason`, `CancelCause` from
`exchange_fill`; `AccountId`, `OrderId` from `exchange_id`; `Order`,
`Obligation` from `exchange_order`; `Sequence` from `exchange_seq`; `Side`
from `exchange_side`), plus this crate's own now-dropped `Amount` alias
(callers needing it now reach `exchange_order::Amount` directly), are gone
from this surface entirely — see each one's own crate for its current
`docs/item/`.

### Not part of the 23-crate proposal

Unlike this family's other crates, `exchange_types` predates the 23-crate
proposal — it is one of the family's original 4 real crates, so there is
no `docs/crate/`, `docs/exposed_item/`, or `docs/crate_responsibility/`
central entry for it to compare against or thin. Its own fidelity concern
is internal: whether this file and the crate's own
[`readme.md`](../../readme.md) still describe the *current* residual
surface, not a comparison to an external spec. The crate's own module doc
([`src/lib.rs`](../../src/lib.rs)) and readme's "Extraction — shrinking,
not static" section are the authoritative history of what moved out and
when; not repeated here.
